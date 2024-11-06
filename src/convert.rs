use crate::utils::{
    calc::{eval, meval},
    translate::Translator,
    units::Measurement,
};
use anyhow::Result;

/// This is magic function that's used to convert units and translate text
/// Use that **format** for translate `lg to lg text` or `lg:lg text` where `lg` is language code.
/// Use that **format** for convert `nu to u` or `nu:u` where `n` is number and `u` is unit.
pub async fn magic_convert(query: &String) -> Result<String> {
    let measure_response = Measurement::convert(query);
    let tr = Translator::new();
    if let Ok(resp) = measure_response {
        return Ok(resp.txt());
    }
    if let Ok(resp) = eval(query) {
        return Ok(resp.to_string());
    }
    if let Ok(resp) = meval(query) {
        return Ok(resp.txt());
    }
    let translate_response = tr.convert(query).await?;
    Ok(translate_response)
}
