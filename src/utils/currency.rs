use anyhow::{bail, Result};
use regex::Regex;
use serde_json::Value;
#[derive(Debug)]
pub struct Currency {
    pub currency: String,
    pub amount: f64,
}

impl Currency {
    pub fn new(currency: String, amount: f64) -> Self {
        Self { currency, amount }
    }

    pub fn txt(&self) -> String {
        format!("{} {}", self.amount, self.currency)
    }

    pub fn from_str(query: &String) -> Result<Currency> {
        let mut curr = Currency::new(String::new(), 0.);
        let num_str = query
            .chars()
            .take_while(|c| c.is_digit(10) || c == &'.')
            .collect::<String>();
        match num_str.parse::<f64>() {
            Ok(v) => curr.amount = v,
            Err(_) => {}
        };
        let curr_part = query.chars().skip(num_str.len()).collect::<String>();
        if curr_part.is_empty() {
            bail!("Invalid conversion query: error parsing unit part.");
        }
        curr.currency = curr_part;

        Ok(curr)
    }
}

pub async fn curr_convert(from: &str, to: &str, amount: f64) -> Result<Currency> {
    let response = reqwest::get(
        "https://open.er-api.com/v6/latest/".to_owned() + from.to_uppercase().as_str(),
    )
    .await?
    .json::<Value>()
    .await?;
    let rates = response["rates"].as_object();
    if let None = rates {
        bail!("Invalid conversion query: error parsing rates part.");
    }
    let rates = rates.unwrap();
    let rate = rates.get(&to.to_uppercase());
    if let None = rate {
        bail!("Invalid conversion query: error parsing rate part.");
    }
    if let None = rate.unwrap().as_f64() {
        bail!("Invalid conversion query: error parsing rate part.");
    }
    let curr = Currency::new(to.to_string(), amount * rate.unwrap().as_f64().unwrap());
    Ok(curr)
}

pub async fn curr_convert_q(query: &String) -> Result<Currency> {
    let regex = Regex::new(r"(:|to)").unwrap();
    let parts: Vec<&str> = regex.split(query).collect(); //

    if parts.len() != 2 {
        bail!("Invalid conversion query: error parsing parts.");
    }
    let from_part = parts[0].split_whitespace().collect::<String>();
    let to_part = parts[1].split_whitespace().collect::<String>();
    let from = match Currency::from_str(&from_part) {
        Ok(m) => m,
        Err(_) => bail!("Invalid conversion query: error parsing from part."),
    };
    let to = match Currency::from_str(&to_part) {
        Ok(m) => m,
        Err(_) => bail!("Invalid conversion query: error parsing to part"),
    };
    let curr = curr_convert(&from.currency, &to.currency, from.amount).await?;

    Ok(curr)
}
