use std::error::Error;
use std::os::unix::process::CommandExt;
use std::process::Command;

mod args;
mod parameter_store;
mod program_env;

use parameter_store::ParameterStore;
use program_env::ProgramEnv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = args::parse();
    let store = ParameterStore::new().await;

    let mut env = ProgramEnv::new();
    for path in args.paths {
        let params = store.list_parameters(&path).await?;
        env.params(&params);
    }

    if !args.clean_env {
        env.vars(&std::env::vars().collect::<Vec<(String,String)>>());
    }

    let cmd = args.program.first().unwrap();
    Command::new(cmd)
        .env_clear()
        .envs(env.to_map())
        .args(args.program.iter().skip(1))
        .exec();

    return Ok(());
}
