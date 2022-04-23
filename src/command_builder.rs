use std::{
    collections::HashMap,
    process::Command,
};

pub struct CommandBuilder {
    program: Vec<String>,
    params: HashMap<String, String>,
    env_vars: Vec<(String, String)>,
}

impl CommandBuilder {
    pub fn new(program: &[&str]) -> Self {
        return Self {
            program: program.into_iter().map(|s| s.to_string()).collect(),
            params: HashMap::new(),
            env_vars: Vec::new(),
        };
    }

    pub fn with_params(&mut self, params: &[(&str, &str)]) -> &mut Self {
        for (k, v) in params {
            self.params.insert(
                k.to_uppercase().replace("-", "_").replace("/", "_"),
                v.to_string(),
            );
        }
        return self;
    }

    pub fn with_env_vars(&mut self, env_vars: &[(&str, &str)]) -> &mut Self {
        for (k, v) in env_vars {
            self.env_vars.push((k.to_string(), v.to_string()));
        }
        return self;
    }

    pub fn build(&self) -> Command {
        let mut envs = self.params.clone();
        for (k, v) in &self.env_vars {
            envs.insert(k.to_string(), v.to_string());
        }
        let mut cmd = Command::new(self.program.first().unwrap());
        cmd.args(self.program.iter().skip(1));
        cmd.env_clear();
        cmd.envs(envs);
        return cmd;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    trait TestCommand {
        fn env_vars_map(&self) -> HashMap<&str, &str>;
        fn args_vec(&self) -> Vec<&str>;
    }
    impl TestCommand for Command {
        fn env_vars_map(&self) -> HashMap<&str, &str> {
            self.get_envs()
                .map(|(k, v)| (k.to_str().unwrap(), v.unwrap().to_str().unwrap()))
                .collect()
        }

        fn args_vec(&self) -> Vec<&str> {
            self.get_args()
                .map(|a|a.to_str().unwrap())
                .collect()
        }
    }

    fn map_of<'a>(values: &[(&'a str, &'a str)]) -> HashMap<&'a str, &'a str> {
        let mut m = HashMap::new();
        for (k, v) in values {
            m.insert(*k, *v);
        }
        return m;
    }

    #[test]
    fn program() {
        let cmd = CommandBuilder::new(&["prg"]).build();

        assert_eq!(cmd.get_program(), "prg");
    }

    #[test]
    fn program_with_args() {
        let cmd = CommandBuilder::new(&["prg", "arg1", "arg2"]).build();

        assert_eq!(cmd.args_vec(), vec!("arg1", "arg2"));
    }

    #[test]
    fn empty() {
        let cmd = CommandBuilder::new(&["env"]).build();

        assert_eq!(cmd.env_vars_map(), map_of(&vec!()));
    }

    #[test]
    fn with_params() {
        let cmd = CommandBuilder::new(&["env"])
            .with_params(&vec![("user-name", "user"), ("password", "pass")])
            .build();

        assert_eq!(
            cmd.env_vars_map(),
            map_of(&vec!(("USER_NAME", "user"), ("PASSWORD", "pass"),))
        );
    }

    #[test]
    fn with_nested_params() {
        let cmd = CommandBuilder::new(&["env"])
            .with_params(&vec![
                ("a/nested/parameter", "nested"),
                ("not-nested", "not-nested"),
            ])
            .build();

        assert_eq!(
            cmd.env_vars_map(),
            map_of(&vec!(
                ("A_NESTED_PARAMETER", "nested"),
                ("NOT_NESTED", "not-nested"),
            ))
        );
    }

    #[test]
    fn multiple_param_sets_are_merged() {
        let cmd = CommandBuilder::new(&["env"])
            .with_params(&[("user-name", "user"), ("password", "pass")])
            .with_params(&[("merged", "value1")])
            .build();

        assert_eq!(
            cmd.env_vars_map(),
            map_of(&vec!(
                ("USER_NAME", "user"),
                ("PASSWORD", "pass"),
                ("MERGED", "value1"),
            ))
        );
    }

    #[test]
    fn overlapping_param_sets_uses_last_value() {
        let cmd = CommandBuilder::new(&["env"])
            .with_params(&[("user-name", "user"), ("password", "pass")])
            .with_params(&[("password", "override")])
            .build();

        assert_eq!(
            cmd.env_vars_map(),
            map_of(&vec!(("USER_NAME", "user"), ("PASSWORD", "override"),))
        );
    }

    #[test]
    fn with_env() {
        let cmd = CommandBuilder::new(&["env"])
            .with_env_vars(&[("ENV1", "one"), ("ENV2", "two")])
            .build();

        assert_eq!(
            cmd.env_vars_map(),
            map_of(&vec!(("ENV1", "one"), ("ENV2", "two"),))
        );
    }

    #[test]
    fn env_takes_precedence_over_params() {
        let cmd = CommandBuilder::new(&["env"])
            .with_env_vars(&[("PASSWORD", "from-env")])
            .with_params(&[("user-name", "user"), ("password", "pass")])
            .build();

        assert_eq!(
            cmd.env_vars_map(),
            map_of(&vec!(("USER_NAME", "user"), ("PASSWORD", "from-env"),))
        );
    }
}
