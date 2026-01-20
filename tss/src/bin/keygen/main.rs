use anyhow::{anyhow, Context, Result,bail};
use futures::StreamExt;
use std::path::PathBuf;
use structopt::StructOpt;

use multi_party_ecdsa::protocols::multi_party_ecdsa::gg_2020::state_machine::keygen::Keygen;
use round_based::async_runtime::AsyncProtocol;

use tss::client::join_computation;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short, long, default_value = "http://localhost:8000/")]
    address: surf::Url,
    #[structopt(short, long, default_value = "default-keygen")]
    room: String,
    #[structopt(short, long)]
    threshold: u16,
    #[structopt(short, long)]
    number_of_parties: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Cli = Cli::from_args();
    let mut filename = PathBuf::from(format!("./keys/{}",args.room));
    if !filename.exists() {
        let _ = std::fs::create_dir_all(&filename)?;
    }
    

    let (index, incoming, outgoing) = join_computation(args.address, &args.room)
        .await
        .context("join computation")?;
    println!("index: {}", index);
    // 파일 이름 변경
    filename.push(format!("{}.json", index));

    let incoming = incoming.fuse();
    tokio::pin!(incoming);
    tokio::pin!(outgoing);

    let keygen = Keygen::new(index, args.threshold, args.number_of_parties)?;
    let output = AsyncProtocol::new(keygen, incoming, outgoing)
        .run()
        .await
        .map_err(|e| anyhow!("protocol execution terminated with error: {}", e))?;
    println!("output: {:?}", output);
    let result = serde_json::to_vec_pretty(&output).context("serialize output")?;
    
    let mut output_file = tokio::fs::OpenOptions::new()
    .write(true)
    .create_new(true)
    .open(filename)
    .await
    .context("cannot create output file")?;
    tokio::io::copy(&mut result.as_slice(), &mut output_file)
        .await
        .context("save output to file")?;


    Ok(())
}
