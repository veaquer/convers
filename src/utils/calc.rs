use super::units::Measurement;
use anyhow::Result;

/// Evaluates a mathematical expression and returns the result.
pub fn eval(query: &String) -> Result<f64> {
    let parts: Vec<&str> = query
        .split_inclusive(|c| c == '+' || c == '-' || c == '/' || c == '*')
        .collect();

    let mut result: f64 = parts[0]
        .split(|c| c == '+' || c == '-' || c == '/' || c == '*')
        .collect::<String>()
        .parse::<f64>()?;

    for i in 1..parts.len() {
        let part = parts[i];
        let operator = parts[i - 1]
            .chars()
            .filter(|c| c == &'+' || c == &'-' || c == &'*' || c == &'/')
            .last()
            .unwrap_or(' ');
        let next_operator = parts[i]
            .chars()
            .filter(|c| c == &'+' || c == &'-' || c == &'*' || c == &'/')
            .last()
            .unwrap_or(' ');
        let number = part[..part.len()]
            .replace(next_operator, "")
            .parse::<f64>()?;
        match operator {
            '+' => result += number,
            '-' => result -= number,
            '*' => result *= number,
            '/' => result /= number,
            _ => {}
        }
    }

    Ok(result)
}

/// Evaluates a mathematical expression with Unit and returns the result.
pub fn meval(query: &String) -> Result<Measurement> {
    let parts: Vec<&str> = query
        .split_inclusive(|c| c == '+' || c == '-' || c == '/' || c == '*')
        .collect();

    let mut result = Measurement::from_str(
        &parts[0]
            .split(|c| c == '+' || c == '-' || c == '/' || c == '*')
            .collect::<String>(),
    )
    .map(|x| *x)?;

    for i in 1..parts.len() {
        let part = parts[i];
        let operator = parts[i - 1]
            .chars()
            .filter(|c| c == &'+' || c == &'-' || c == &'*' || c == &'/')
            .last()
            .unwrap_or(' ');
        let next_operator = parts[i]
            .chars()
            .filter(|c| c == &'+' || c == &'-' || c == &'*' || c == &'/')
            .last()
            .unwrap_or(' ');
        let measur =
            Measurement::from_str(&part[..part.len()].replace(next_operator, "")).map(|x| *x)?;

        match operator {
            '+' => result = (result + measur)?,
            '-' => result = (result - measur)?,
            '*' => result = (result * measur)?,
            '/' => result = (result / measur)?,
            _ => {}
        }
    }

    Ok(result)
}
