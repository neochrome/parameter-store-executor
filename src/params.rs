pub fn transform(name: &str, prefix: &str) -> String {
    (if name.starts_with(prefix) {
        name.replacen(prefix, "", 1)
    } else {
        String::from(name)
    })
    .to_uppercase()
    .replace("-", "_")
    .replace("/", "_")
}

#[derive(Debug)]
pub struct Parameter {
    name: String,
    value: String,
}

use aws_sdk_ssm::output::GetParametersByPathOutput as Output;
use aws_sdk_ssm::Client;
use std::error::Error;
use tokio_stream::StreamExt;

pub async fn list_by_path(client: &Client, path: &str) -> Result<Vec<Parameter>, Box<dyn Error>> {
    let result: Result<Vec<Output>, _> = client
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
                    name: String::from(p.name().unwrap()),
                    value: String::from(p.value().unwrap()),
                })
                .collect();
            Ok(params)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_upper() {
        assert_eq!(transform("/something", "/"), "SOMETHING");
    }

    #[test]
    fn to_snake() {
        assert_eq!(transform("/a-value", "/"), "A_VALUE");
    }

    #[test]
    fn flattens() {
        assert_eq!(transform("/a/path/to/a/value", "/"), "A_PATH_TO_A_VALUE");
    }
}
