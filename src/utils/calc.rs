use std::num::ParseFloatError;

pub fn eval(query: &String) -> Result<f64, ParseFloatError> {
    let parts: Vec<&str> = query
        .split_inclusive(|c| c == '+' || c == '-' || c == '/' || c == '*')
        .collect();
    let mut result: f64 = parts[0]
        .split(|c| c == '+' || c == '-' || c == '/' || c == '*')
        .collect::<String>()
        .parse::<f64>()?;
    println!("Initializated num:{result}");
    for i in 1..parts.len() {
        let part = parts[i];
        let operator = parts[i - 1]
            .chars()
            .filter(|c| c == &'+' || c == &'-' || c == &'*' || c == &'/')
            .last()
            .unwrap_or(' ');
        println!("Op:{}", operator);
        let next_operator = parts[i]
            .chars()
            .filter(|c| c == &'+' || c == &'-' || c == &'*' || c == &'/')
            .last()
            .unwrap_or(' ');
        let number = part[..part.len()]
            .replace(next_operator, "")
            .parse::<f64>()?;
        println!("Num:{}", number);
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
