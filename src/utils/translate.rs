use anyhow::{bail, Context, Result};
use regex_split::RegexSplit;
use serde_json::Value;
use std::{fmt, io::Error};

#[derive(Debug)]
pub enum ConvertError {
    ReqwestError(Error),
    CustomError(String),
}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConvertError::ReqwestError(e) => write!(f, "Reqwest error: {}", e),
            ConvertError::CustomError(e) => write!(f, "Custom error: {}", e),
        }
    }
}

impl From<Error> for ConvertError {
    fn from(error: Error) -> Self {
        ConvertError::ReqwestError(error)
    }
}

pub struct Translator {}

impl Translator {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn translate(&self, from: &str, to: &str, text: &str) -> Result<String> {
        let url = format!(
            "https://translate.googleapis.com/translate_a/single?client=gtx&sl={}&tl={}&dt=t&q={}",
            from, to, text
        );

        let response = reqwest::get(&url).await?.json::<Value>().await?;
        if let Some(array) = response.as_array() {
            if let Some(nested_array) = array.get(0).and_then(|v| v.as_array()) {
                if let Some(inner_array) = nested_array.get(0).and_then(|v| v.as_array()) {
                    let str_vec: Vec<String> = inner_array
                        .iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect();
                    if str_vec.is_empty() {
                        bail!("Invalid response");
                    }

                    return Ok(format!("\n [ {} -> {} ] \n\n {}", from, to, str_vec[0]));
                }
            }
        }
        bail!("Invalid response")
    }

    /// Translates text from &String query.
    /// Example of query: `en to ru how are you?`.
    pub async fn convert(&self, text: &String) -> anyhow::Result<String> {
        let re = regex::Regex::new(r"(:|to)").unwrap();
        let parts: Vec<&str> = re.split_inclusive(text).collect();
        if parts.len() < 2 {
            bail!("Invalid conversion query: error parsing parts.");
        }
        let from_part = parts[0]
            .split_whitespace()
            .collect::<Vec<&str>>()
            .get(0)
            .unwrap_or(&"en")
            .to_string()
            .replace(":", "");
        let second_part: Vec<&str> = parts[1].split_whitespace().collect();
        let to_part = second_part[0];
        let text_part: String = parts
            .iter()
            .skip(1)
            .map(|x| *x)
            .collect::<Vec<&str>>()
            .join(" ")
            .split_whitespace()
            .collect::<Vec<&str>>()
            .get(1..)
            .unwrap_or(&Vec::new())
            .join(" ");

        self.translate(&from_part, to_part, &text_part)
            .await
            .context("Translation failed")
    }
}
