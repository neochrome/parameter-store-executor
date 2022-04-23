mod args;
mod parameter_store;
use parameter_store::ParameterStore;

use std::{collections::HashMap, error::Error};

use std::os::unix::process::CommandExt;
use std::process::Command;

mod command_builder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = args::parse();
    let store = ParameterStore::new().await;

    let mut params = HashMap::new();

    for path in args.paths {
        for param in store.list_parameters(&path).await? {
            params.insert(param.name, param.value);
        }
    }

    if !args.clean_env {
        for (name, value) in std::env::vars() {
            params.insert(name, value);
        }
    }

    let cmd = args.command.pop_front().unwrap();
    Command::new(cmd)
        .env_clear()
        .envs(params)
        .args(args.command)
        .exec();

    return Ok(());
}
