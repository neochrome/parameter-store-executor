#![allow(unused)]

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ssm::{Client, Region};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // args::parse();

    let region_provider = RegionProviderChain::default_provider()
        .or_else(Region::new("eu-west-1"));
    let config = aws_config::from_env().load().await; //.region(region_provider).load().await;
    let client = Client::new(&config);
    let params = params::list_by_path(&client, "/").await?;
    for p in params {
        println!("{:?}", p);
    }

    return Ok(());
}

mod params;
mod args;
