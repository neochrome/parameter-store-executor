#![allow(unused)]

mod args;
mod parameter_store;
use parameter_store::ParameterStore;

use std::{error::Error, collections::HashMap};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = args::parse();
    let store = ParameterStore::new().await;

    let mut params = HashMap::new();

    for path in args.paths {
        for param in store.list_parameters(&path).await? {
            params.insert(param.name, param.value);
        }
    }

    if (!args.clean_env) {
        for (name,value) in std::env::vars() {
            params.insert(name, value);
        }
    }

    println!("{:?}", params);

    return Ok(());
}
