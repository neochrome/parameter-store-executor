#![allow(unused)]

use std::{
    collections::VecDeque,
    env::ArgsOs,
    ffi::{OsStr, OsString},
};

use clap::{arg, builder::NonEmptyStringValueParser, command, Arg, ArgAction, Command};

#[derive(Debug, PartialEq, Eq)]
pub struct Args {
    pub paths: Vec<String>,
    pub program: String,
    pub program_args: Vec<String>,
    pub clean_env: bool,
}

fn build_parser() -> clap::Command<'static> {
    command!()
        .author("")
        .term_width(80)
        .long_about(
            "Fetches parameters recursively from AWS SSM Parameter Store at the given PARAMETER_PATH(s).\n\
            Then executes PROGRAM with the parameters supplied as ENV variables."
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
                    "Path(s) into AWS SSM Parameter Store, must begin with a '/'\n\
                    \n\
                    E.g: /my-app/secrets",
                )
                .required(true)
                .value_parser(|p: &str|
                    if p.starts_with('/') {
                        Ok(p.to_string())
                    } else {
                        Err(String::from("The parameter must start with a '/'"))
                    }
                )
                .multiple_values(true)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("program")
                .value_name("PROGRAM [ARG]...")
                .help("The PROGRAM to execute")
                .long_help(
                    "The PROGRAM, with optional ARG(s), to execute",
                )
                .required(false)
                .default_value("env")
                .multiple_values(false)
                .last(true),
        )
        .after_long_help("\
            NOTES:\n\
            Before passing the parameters to the PROGRAM, their names will be transformed as follows:\n\
            - Remove PARAMETER_PATH prefix\n\
            - Replace the symbols . (period), - (hyphen) and / (forward slash) with _ (underscore)\n\
            - Made UPPERCASE\n\
            \n\
            Conflicting parameters will resolve to the value of the last one found.\n\
            Any existing ENV variables (unless --clean-env is specified) will be passed \
            along and takes precedence over parameters with the same name - to allow \
            overriding specific parameters (e.g in development environment).\n\
            \n\
            Example:\n\
               Given the following parameters:\n\
               | name           | value  |\n\
               +----------------+--------+\n\
               | /one/user-name | user-1 |\n\
               | /one/password  | pass-1 |\n\
               | /two/user-name | user-2 |\n\
               | /two/password  | pass-2 |\n\
            \n\
               And the following existing ENV vars\n\
               | name         | value    |\n\
               +--------------+----------+\n\
               | TWO_PASSWORD | from-env |\n\
            \n\
               When requesting: [/, /one, /two]\n\
            \n\
               Then the following ENV variables will be available:\n\
               | name          | value    | comment                  |\n\
               +---------------+----------+--------------------------+\n\
               | ONE_USER_NAME | user-1   | /                        |\n\
               | ONE_PASSWORD  | pass-1   | /                        |\n\
               | TWO_USER_NAME | user-2   | /                        |\n\
               | TWO_PASSWORD  | from-env | /, superceded by ENV var |\n\
               | USER_NAME     | user-2   | /one, superceded by /two |\n\
               | PASSWORD      | pass-2   | /one, superceded by /two |\n\
            "
        )
}

pub fn parse() -> Args {
    parse_from(std::env::args_os())
}

pub fn parse_from<I, T>(input: I) -> Args
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let args = build_parser().get_matches_from(input);

    let paths: Vec<String> = args.get_many("paths").unwrap().cloned().collect();
    let program_and_args: Vec<String> = args.get_many("program").unwrap().cloned().collect();
    let clean_env = args.contains_id("clean-env");

    Args {
        paths,
        program: program_and_args.first().unwrap().clone(),
        program_args: program_and_args.into_iter().skip(1).collect(),
        clean_env,
    }
}

#[test]
fn verify_parser() {
    build_parser().debug_assert();
}

#[test]
fn complete_example() {
    let args = parse_from(vec![
        "pse",
        "--clean-env",
        "/a/path",
        "/another/path",
        "--",
        "/path/to/a/binary",
        "arg1",
        "arg2",
    ]);
    assert_eq!(
        args,
        Args {
            paths: vec!["/a/path", "/another/path"]
                .into_iter()
                .map(Into::into)
                .collect(),
            program: "/path/to/a/binary".to_string(),
            program_args: vec!["arg1", "arg2"].into_iter().map(Into::into).collect(),
            clean_env: true,
        }
    )
}
