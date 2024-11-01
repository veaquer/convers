use std::fmt;

use reqwest::Error;
use serde_json::Value;

use super::CONV_ERROR;

#[derive(Debug)]
pub enum TranslatorError {
    ReqwestError(Error),
    CustomError(String),
}

impl fmt::Display for TranslatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TranslatorError::ReqwestError(e) => write!(f, "Reqwest error: {}", e),
            TranslatorError::CustomError(e) => write!(f, "Custom error: {}", e),
        }
    }
}

impl From<Error> for TranslatorError {
    fn from(error: Error) -> Self {
        TranslatorError::ReqwestError(error)
    }
}

pub struct Translator {}

impl Translator {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn translate(
        &self,
        from: &str,
        to: &str,
        text: &str,
    ) -> Result<String, TranslatorError> {
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
                        return Err(TranslatorError::CustomError("Invalid response".to_string()));
                    }

                    return Ok(format!("\n [ {} -> {} ] \n\n {}", from, to, str_vec[0]));
                }
            }
        }
        Err(TranslatorError::CustomError("Invalid response".to_string()))
    }

    /// Translates text from &String query.
    /// Example of query: `en to ru how are you?`.
    pub async fn convert(&self, text: &String) -> Result<String, TranslatorError> {
        let parts: Vec<&str> = text.split_whitespace().collect();
        let conv_sign_index = match parts.iter().position(|&x| x == "to" || x == ":") {
            Some(i) => i,
            None => return Err(TranslatorError::CustomError(CONV_ERROR.to_string())),
        };
        if parts.len() < 4 {
            return Err(TranslatorError::CustomError(CONV_ERROR.to_string()));
        }
        let from_part = parts[..conv_sign_index].join(" ");
        let to_part = parts[conv_sign_index + 1];
        let text_part = parts[conv_sign_index + 2..].join(" ");
        self.translate(&from_part, to_part, &text_part).await
    }
}