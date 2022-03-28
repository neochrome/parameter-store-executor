#![allow(unused)]

use clap::Parser;
/// Fetches parameters recursively from AWS SSM Parameter Store at the given PARAMETER_PATH(s).
/// Then COMMAND is executed with the parameters supplied as ENV variables.
/// The ENV variables will be named as the parameters, with the following transformation applied:
/// - UPPERCASEd
/// - Made relative to the corresponding PARAMETER_PATH
/// - All '/' & '-' characters replaced with '_'
#[derive(Parser, Debug)]
#[clap(version, about, verbatim_doc_comment)]
struct CLI {
    /// Don't pass any existing ENV variables
    #[clap(short, long)]
    clean_env: bool,

    /// Path(s) into AWS SSM Parameter Store, must begin with a '/'.
    /// E.g: /my-app/secrets.
    // #[clap(required = true, multiple_values = true, short, long, name = "PARAMETER_PATH")]
    #[clap(
        required = true,
        forbid_empty_values = true,
        validator = is_a_parameter_path,
        name = "PARAMETER_PATH",
        verbatim_doc_comment
    )]
    parameter_paths: Vec<String>,

    /// The COMMAND, with optional ARG(s), to invoke with the parameters as ENV vars.
    #[clap(
        raw = true,
        default_value = "env",
        multiple_values = false,
        multiple_occurrences = false,
        name = "COMMAND [ARG]..."
    )]
    command: Vec<String>,
}

fn is_a_parameter_path(v: &str) -> Result<(), String> {
    if v.starts_with('/') {
        return Ok(());
    } else {
        return Err(String::from("The parameter must start with a '/'"));
    }
}

fn main() {
    let cli = CLI::parse();
    println!("{:?}", cli);

    // std::process::exit(0);
}
