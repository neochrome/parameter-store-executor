use std::os::unix::process::CommandExt;
use std::process::Command;

mod args;
mod parameter_store;
mod program_env;

use parameter_store::ParameterStore;
use program_env::ProgramEnv;

#[tokio::main]
async fn main() {
    let args = args::parse();
    let store = ParameterStore::new().await;

    let mut env = ProgramEnv::new();
    for path in args.paths {
        let result = store.list_parameters(&path).await;
        match result {
            Ok(params) => env.params(&params),
            Err(error) => {
                eprintln!("[ERROR] Reading parameters at `{}`. {}", path, error);
                std::process::exit(1);
            },
        };
    }

    if !args.clean_env {
        env.vars(&std::env::vars().collect::<Vec<_>>());
    }

    let error = Command::new(&args.program)
        .env_clear()
        .envs(&env.to_map())
        .args(&args.program_args)
        .exec();

    eprintln!("[ERROR] Executing `{}`. {}", &args.program, error.to_string());
    std::process::exit(1);
}
