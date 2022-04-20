#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub value: String,
}
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ssm::{Client, Region};

use aws_sdk_ssm::output::GetParametersByPathOutput as Output;

use std::collections::HashMap;
use std::error::Error;
use tokio_stream::StreamExt;

pub struct ParameterStore {
    client: Client,
}

impl ParameterStore {
    pub async fn new() -> Self {
        let region_provider =
            RegionProviderChain::default_provider().or_else(Region::new("eu-west-1"));
        let config = aws_config::from_env().load().await; //.region(region_provider).load().await;
        let client = Client::new(&config);
        ParameterStore { client }
    }

    pub async fn list_parameters(&self, path: &str) -> Result<Vec<Parameter>, Box<dyn Error>> {
        let result: Result<Vec<Output>, _> = self
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
                        name: transform_name(p.name().unwrap(), path),
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
        String::from(name)
    }
}

fn transform_name(name: &str, path: &str) -> String {
    make_relative(name, path)
    .to_uppercase()
    .replace("-", "_")
    .replace("/", "_")
}

#[cfg(test)]
mod tests {
    use super::*;

    mod transform_name {
        use super::*;

        #[test]
        fn to_upper() {
            assert_eq!(transform_name("/something", "/"), "SOMETHING");
        }

        #[test]
        fn to_snake() {
            assert_eq!(transform_name("/a-value", "/"), "A_VALUE");
        }

        #[test]
        fn makes_relative() {
            assert_eq!(transform_name("/a/path/to/a/value", "/a/path/to"), "A_VALUE");
        }
    }
}
