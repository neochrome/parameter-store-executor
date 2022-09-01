use crate::parameter_store::Parameter;
use std::collections::HashMap;

#[derive(Default)]
pub struct ProgramEnv {
    params: HashMap<String, String>,
    vars: Vec<(String, String)>,
}

impl ProgramEnv {
    pub fn new() -> ProgramEnv {
        ProgramEnv::default()
    }

    pub fn params(&mut self, params: &[Parameter]) -> &mut ProgramEnv {
        for p in params {
            self.params.insert(
                p.name
                    .to_ascii_uppercase()
                    .replace("-", "_")
                    .replace("/", "_"),
                p.value.to_string(),
            );
        }
        self
    }

    pub fn vars(&mut self, vars: &[(String, String)]) -> &mut ProgramEnv {
        self.vars = vars.to_vec();
        self
    }

    pub fn to_map(&mut self) -> HashMap<String, String> {
        let mut env = self.params.clone();
        for (k, v) in self.vars.clone() {
            env.insert(k, v);
        }
        env
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{map, params, vars};

    #[test]
    fn empty() {
        let env = ProgramEnv::new().to_map();

        assert_eq!(env, HashMap::new());
    }

    #[test]
    fn with_params() {
        let env = ProgramEnv::new()
            .params(&params![("user-name", "user"), ("password", "pass")])
            .to_map();

        assert_eq!(env, map![("USER_NAME", "user"), ("PASSWORD", "pass")]);
    }

    #[test]
    fn with_nested_params() {
        let env = ProgramEnv::new()
            .params(&params![
                ("a/nested/parameter", "nested"),
                ("not-nested", "not-nested"),
            ])
            .to_map();

        assert_eq!(
            env,
            map![
                ("A_NESTED_PARAMETER", "nested"),
                ("NOT_NESTED", "not-nested"),
            ]
        );
    }

    #[test]
    fn multiple_sets_of_parameters_are_merged() {
        let env = ProgramEnv::new()
            .params(&params![("user-name", "user"), ("password", "pass")])
            .params(&params![("merged", "value1")])
            .to_map();

        assert_eq!(
            env,
            map![
                ("USER_NAME", "user"),
                ("PASSWORD", "pass"),
                ("MERGED", "value1"),
            ]
        );
    }

    #[test]
    fn overlapping_sets_of_parameters_uses_last_value() {
        let env = ProgramEnv::new()
            .params(&params![("user-name", "user"), ("password", "pass")])
            .params(&params![("password", "override")])
            .to_map();

        assert_eq!(env, map![("USER_NAME", "user"), ("PASSWORD", "override")]);
    }

    #[test]
    fn with_env_vars() {
        let env = ProgramEnv::new()
            .vars(&vars![("ENV1", "one"), ("ENV2", "two")])
            .to_map();

        assert_eq!(env, map![("ENV1", "one"), ("ENV2", "two")]);
    }

    #[test]
    fn env_vars_takes_precedence_over_parameters() {
        let env = ProgramEnv::new()
            .vars(&vars![("PASSWORD", "from-env")])
            .params(&params![("user-name", "user"), ("password", "pass")])
            .to_map();

        assert_eq!(env, map![("USER_NAME", "user"), ("PASSWORD", "from-env")]);
    }

    #[macro_export]
    macro_rules! params {
        ( $( $x:expr ),* $(,)* ) => {
            {
                let mut temp = Vec::new();
                $(
                    temp.push(Parameter{ name: $x.0.to_string(), value: $x.1.to_string() });
                )*
                temp
            }
        };
    }

    #[macro_export]
    macro_rules! vars {
        ( $( $x:expr ),* $(,)* ) => {
            {
                let mut temp = Vec::new();
                $(
                    temp.push(($x.0.to_string(), $x.1.to_string()));
                )*
                temp
            }
        };
    }

    #[macro_export]
    macro_rules! map {
        ( $( $x:expr ),* $(,)* ) => {
            {
                let mut temp = HashMap::new();
                $(
                    temp.insert($x.0.to_string(), $x.1.to_string());
                )*
                temp
            }
        };
    }
}
