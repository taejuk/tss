use ethers::types::{Signature, U256};
use ethers::{prelude::*, utils::parse_ether};
use multi_party_ecdsa::protocols::multi_party_ecdsa::gg_2020::party_i::SignatureRecid;

use std::str::FromStr;
use anyhow::{Context, Result, bail};
use ethers::types::transaction::eip2718::TypedTransaction;

use ethers::utils::keccak256;
use k256::ecdsa::VerifyingKey;

use ethers::utils::rlp::Rlp;
use ethers::types::Transaction;
use ethers::utils::rlp::Decodable;

// 모두가 같은 서명을 해야 한다.

pub async fn make_transaction(provider: &Provider<Http>,_from:&str, _to: &str, _ether: &str) -> Result<Eip1559TransactionRequest> {
    let from = Address::from_str(_from).context("Error: convert from address")?;
    let to = Address::from_str(_to).context("Error: convert to address")?;
    let ether = parse_ether(_ether).context("Error: conver ether")?;
    let nonce = provider.get_transaction_count(from, None).await.context("Error: get nonce")?;
    let chain_id = provider.get_chainid().await.context("Error: get chain id")?;
    let (max_fee_per_gas, max_priority_fee_per_gas) = provider.estimate_eip1559_fees(None).await.context("Error: get fee info")?;

    let mut tx = Eip1559TransactionRequest::new()
        .to(to)
        .value(ether)
        .from(from)          
        .nonce(nonce)              
        .chain_id(chain_id.as_u64()) 
        .max_fee_per_gas(max_fee_per_gas)
        .max_priority_fee_per_gas(max_priority_fee_per_gas);

    
    let typed_tx: TypedTransaction = tx.clone().into();
    let gas_limit = provider.estimate_gas(&typed_tx, None).await?;
    tx = tx.gas(gas_limit);

    Ok(tx)
}
pub async fn make_transaction_fixed_gas(provider: &Provider<Http>,_from:&str, _to: &str, _ether: &str) -> Result<Eip1559TransactionRequest> {
    let from = Address::from_str(_from).context("Error: convert from address")?;
    let to = Address::from_str(_to).context("Error: convert to address")?;
    let ether = parse_ether(_ether).context("Error: conver ether")?;
    let nonce = provider.get_transaction_count(from, None).await.context("Error: get nonce")?;
    let chain_id = provider.get_chainid().await.context("Error: get chain id")?;
    let max_fee_per_gas = U256::from_str("5059808370")?;
    let gas_limit = 100_000;
    let max_priority_fee_per_gas = U256::from_str("3000000000")?;
    let mut tx = Eip1559TransactionRequest::new()
        .to(to)
        .value(ether)
        .from(from)          
        .nonce(nonce)              
        .chain_id(chain_id.as_u64()) 
        .max_fee_per_gas(max_fee_per_gas)
        .max_priority_fee_per_gas(max_priority_fee_per_gas)
        .gas(gas_limit);

    
    let typed_tx: TypedTransaction = tx.clone().into();
    let gas_limit = provider.estimate_gas(&typed_tx, None).await?;
    tx = tx.gas(gas_limit);

    Ok(tx)
}


pub fn get_sighash_to_sign(tx_request: Eip1559TransactionRequest) -> Result<String> {
    let typed_tx: TypedTransaction = tx_request.into();

    let hash: H256 = typed_tx.sighash();
    let sighash = hex::encode(hash.to_fixed_bytes());
    Ok(sighash)
}

pub fn recover_address_from_bytes(signed_tx_bytes: &[u8]) -> Result<Address> {
    // 1. RLP 디코딩 (Bytes -> Transaction 객체)
    // 이 과정에서 바이트 구조가 올바른지 1차 확인이 됩니다.
    let rlp = Rlp::new(signed_tx_bytes);
    let tx: Transaction = Transaction::decode(&rlp)
        .context("RLP 디코딩 실패: 트랜잭션 형식이 잘못되었습니다.")?;

    // 2. 주소 복구 (Recover)
    // 내부적으로 서명(r,s,v)과 메시지 해시를 이용해 보낸 사람을 계산합니다.
    let from_address = tx.recover_from()
        .context("서명 복구 실패: 유효하지 않은 서명입니다.")?;

    Ok(from_address)
}

pub fn bytes_to_address_string(pubkey_bytes: &[u8]) -> Result<String> {
    
    let uncompressed_bytes = match pubkey_bytes.len() {
        65 => {
            if pubkey_bytes[0] != 0x04 {
                bail!("Error: from format")
            }
            pubkey_bytes.to_vec()
        },
        
        33 => {
            let key = VerifyingKey::from_sec1_bytes(pubkey_bytes)
                .context("Error: from format")?;
            
            // false = 비압축(Uncompressed)
            key.to_encoded_point(false).as_bytes().to_vec()
        },
        
        _ => bail!("Error: from format")
    };

    let pubkey_unprefixed = &uncompressed_bytes[1..];
    
    let hash = keccak256(pubkey_unprefixed);
    
    let address_bytes = &hash[12..]; // 32 - 20 = 12부터 끝까지
    
    let address_string = format!("0x{}", hex::encode(address_bytes));
    
    Ok(address_string)
}

pub fn get_signature_for_ethereum(signature: &SignatureRecid) -> Signature {
    let r_bytes = signature.r.to_bytes(); 
    let s_bytes = signature.s.to_bytes();

    let r = U256::from_big_endian(r_bytes.as_ref());
    let s = U256::from_big_endian(s_bytes.as_ref());
    let v = signature.recid as u64;

    Signature { r, s, v }
}
