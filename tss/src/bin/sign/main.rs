use core::num;
// local key를 가져오고
use std::path::PathBuf;

use anyhow::{Context, Result, anyhow, bail};
use ethers::abi::ethereum_types::Signature;
use ethers::providers::Middleware;
use ethers::types::Address;
use ethers::types::transaction::eip2718::TypedTransaction;
use futures::{SinkExt, StreamExt, TryStreamExt};
use structopt::StructOpt;


use curv::arithmetic::Converter;
use curv::BigInt;
use multi_party_ecdsa::protocols::multi_party_ecdsa::gg_2020::state_machine::keygen::LocalKey;
use multi_party_ecdsa::protocols::multi_party_ecdsa::gg_2020::party_i::verify;
use multi_party_ecdsa::protocols::multi_party_ecdsa::gg_2020::state_machine::sign::{
    OfflineStage, SignManual,
};
use round_based::async_runtime::AsyncProtocol;
use round_based::Msg;
use curv::elliptic::curves::Secp256k1;
use tss::client::join_computation;
use dotenv::dotenv;
use std::env;
use tss::provider::get_provider;
use tss::transaction::{bytes_to_address_string, get_sighash_to_sign, get_signature_for_ethereum, make_transaction_fixed_gas};


#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short, long, default_value = "http://localhost:8000/")]
    address: surf::Url,
    #[structopt(short, long, default_value = "default-signing")]
    room: String,
    // 내가 어떤 index인지
    #[structopt(short, long)]
    index: u16,
    // 서명에 참가하는 사람들
    #[structopt(short, long)]
    parties: Vec<u16>,
    
    #[structopt(short, long)]
    to: String,

    #[structopt(short, long)]
    ethers: String
}



#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let sepolia_rpc_url = env::var("SEPOLIA_RPC_URL").context("Error: SEPOLIA RPC URL")?;
    let args: Cli = Cli::from_args();
    let key_json = tokio::fs::read(format!("./keys/{}/{}.json", args.room, args.index)).await.context("Error: can't find key file")?;
    let key: LocalKey<Secp256k1> = serde_json::from_slice(&key_json).context("Error: parse key")?;
    let provider = get_provider(&sepolia_rpc_url)?;
    
    let from = bytes_to_address_string(&key.public_key().to_bytes(false))?;
    
    let tx = make_transaction_fixed_gas(&provider, &from, &args.to, &args.ethers).await?;
    let data_to_sign = get_sighash_to_sign(tx.clone())?;
    // 서명하기 위해 message 변경
    let message = BigInt::from_str_radix(&data_to_sign,16)?;
    let number_of_parties = args.parties.len();
    
    let (i, incoming, outgoing) = join_computation(args.address.clone(), &format!("{}-offline", args.room))
                                    .await.context("Error: join offline computation")?;

    let incoming = incoming.fuse();
    tokio::pin!(incoming);
    tokio::pin!(outgoing);

    let signing = OfflineStage::new(i, args.parties.clone(), key.clone())?;
    let completed_offline_stage = AsyncProtocol::new(signing, incoming, outgoing)
        .run()
        .await
        .map_err(|e| anyhow!("protocol execution terminated with error: {}", e))?;

    let (_i, incoming, outgoing) = join_computation(args.address, &format!("{}-online", args.room))
        .await
        .context("join online computation")?;
    
    tokio::pin!(incoming);
    tokio::pin!(outgoing);
    
    let (signing, partial_signature) = SignManual::new(
        message.clone(),
        completed_offline_stage
    )?;

    outgoing.send(Msg{sender:i, receiver: None, body: partial_signature}).await?;

    let partial_signatures:Vec<_> = incoming.take(number_of_parties-1).map_ok(|msg| msg.body).try_collect().await?;
    let signature = signing.complete(&partial_signatures).context("Error: online stage failed")?;
    let verify_result = verify(&signature, &key.y_sum_s, &message);
    match verify_result {
        Ok(()) => println!("signature valid!"),
        Err(_) => {
            bail!("signature invalid!")
        }
    };
    let sig_ether = get_signature_for_ethereum(&signature);
    let typed_tx:TypedTransaction = tx.clone().into();
    
    let signed_tx_bytes = typed_tx.rlp_signed(&sig_ether);
    
    
    if _i == args.parties[0] {
        let pending_tx = provider.send_raw_transaction(signed_tx_bytes)
        .await
        .context("Error: Pending transaction")?;
    }
    

    Ok(())
}