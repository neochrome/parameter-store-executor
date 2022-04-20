#![allow(unused)]

fn is_a_parameter_path(v: &str) -> Result<(), String> {
    if v.starts_with('/') {
        return Ok(());
    } else {
        return Err(String::from("The parameter must start with a '/'"));
    }
}

use clap::{arg, command, Arg, Command};

pub struct Args {
    pub paths: Vec<String>,
    pub command: Vec<String>,
    pub clean_env: bool,
}

pub fn parse() -> Args {
    let args = command!()
        .author("")
        .long_about(
            "Fetches parameters recursively from AWS SSM Parameter Store at the given PARAMETER_PATH(s).\n\
            Then executes COMMAND with the parameters supplied as ENV variables."
            )
        .arg(
            Arg::new("clean-env")
                .short('c')
                .long("clean-env")
                .help("Start with a clean environment"),
        )
        .arg(
            Arg::new("paths")
                .value_name("PARAMETER_PATH")
                .help("Path(s) into AWS SSM Parameter Store")
                .long_help(
                    "Path(s) into AWS SSM Parameter Store, must begin with a '/'.\n\
                    \n\
                    E.g: /my-app/secrets",
                )
                .required(true)
                .forbid_empty_values(true)
                .multiple_occurrences(true)
                .validator(is_a_parameter_path),
        )
        .arg(
            Arg::new("command")
                .value_name("COMMAND [ARG]...")
                .help("The COMMAND to execute.")
                .long_help(
                    "The COMMAND, with optional ARG(s), to execute with the parameters as ENV vars.",
                )
                .required(false)
                .default_value("env")
                .multiple_occurrences(false)
                .multiple_values(false)
                .last(true),
        )
        .after_long_help("\
            The ENV variables will be named as the parameters, with the following transformation applied:\n \
            - UPPERCASEd\n \
            - Made relative to the corresponding PARAMETER_PATH\n \
            - All '/' and '-' characters replaced with '_'\n\
            \n\
            Conflicting parameters will resolve to the value of the last one found.\n\
            Any existing ENV variables (unless --clean-env is specified) will be passed\
            along and takes precedence over parameters with the same name - to allow\
            overriding specific parameters (e.g in development environment).\n\
            \n\
            Example:\n  \
               Given the following parameters:\n  \
               | name           | value  |\n  \
               +----------------+--------+\n  \
               | /one/user-name | user-1 |\n  \
               | /one/password  | pass-1 |\n  \
               | /two/user-name | user-2 |\n  \
               | /two/password  | pass-2 |\n  \
            \n  \
               And the following existing ENV vars\n  \
               | name         | value    |\n  \
               +--------------+----------+\n  \
               | TWO_PASSWORD | from-env |\n  \
            \n  \
               When requesting: [/, /one, /two]\n  \
            \n  \
               Then the following ENV variables will be available:\n  \
               | name          | value    | comment                  |\n  \
               +---------------+----------+--------------------------+\n  \
               | ONE_USER_NAME | user-1   | /                        |\n  \
               | ONE_PASSWORD  | pass-1   | /                        |\n  \
               | TWO_USER_NAME | user-2   | /                        |\n  \
               | TWO_PASSWORD  | from-env | /                        |\n  \
               | USER_NAME     | user-2   | /one, superceded by /two |\n  \
               | PASSWORD      | pass-2   | /onw, superceded by /two |\n\
            "
        )
        .get_matches();

    let paths: Vec<String> = args.values_of("paths").unwrap().map(String::from).collect();
    let command: Vec<String> = args.values_of("command").unwrap().map(String::from).collect();
    let clean_env: bool = args.is_present("clean-env");

    Args { paths, command, clean_env }
}

