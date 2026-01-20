use ethers::prelude::*;
use std::convert::TryFrom;
use anyhow::{Result, Context};

pub fn get_provider(rpc_url: &str) -> Result<Provider<Http>>{
    Provider::<Http>::try_from(rpc_url).context("Error:fail to get provider")
}