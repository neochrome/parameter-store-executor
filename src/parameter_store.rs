use aws_sdk_ssm::Client;
use aws_sdk_ssm::output::GetParametersByPathOutput;

use std::error::Error;
use tokio_stream::StreamExt;

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub value: String,
}

pub struct ParameterStore {
    client: Client,
}

impl ParameterStore {
    pub async fn new() -> Self {
        let config = aws_config::from_env().load().await;
        let client = Client::new(&config);
        ParameterStore { client }
    }

    pub async fn list_parameters(&self, path: &str) -> Result<Vec<Parameter>, Box<dyn Error>> {
        let result: Result<Vec<GetParametersByPathOutput >, _> = self
            .client
            .get_parameters_by_path()
            .path(path)
            .recursive(true)
            .with_decryption(true)
            .into_paginator()
            .send()
            .collect()
            .await;
        match result {
            Err(e) => Err(Box::new(e)),
            Ok(outputs) => {
                let params = outputs
                    .iter()
                    .flat_map(|o| o.parameters().unwrap())
                    .map(|p| Parameter {
                        name: make_relative(p.name().unwrap(), path),
                        value: String::from(p.value().unwrap()),
                    })
                    .collect();
                Ok(params)
            }
        }
    }
}

fn make_relative(name: &str, path: &str) -> String {
    if name.starts_with(path) {
        make_relative(&name.replacen(path, "", 1), "/")
    } else {
        name.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod transform_name {
        use super::*;

        #[test]
        fn strips_prefix() {
            assert_eq!(make_relative("/a/path/to/a/value", "/a/path/to"), "a/value");
        }
    }
}
