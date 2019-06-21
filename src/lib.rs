use pest::Parser;
use pest_derive::Parser;

use failure::Error;

#[derive(Parser)]
#[grammar = "andaluh.pest"]
pub struct AndaluhParser;

fn keep_case(input: &str, case: &str) -> String {
    input.chars().zip(case.chars())
        .map(|(i, c)| {
            match c.is_uppercase() {
                true => i.to_uppercase().to_string(),
                false => i.to_lowercase().to_string(),
            }
        }).collect::<String>()
}

pub fn epa() {
}

pub fn h_rule(input: &str) -> Result<String, Error> {
        let pairs = AndaluhParser::parse(Rule::h, input)?;
        let mut output: Vec<String> = vec![];

        for pair in pairs {
            let chunk = match pair.as_rule() {
                Rule::initial_h => String::from(&pair.as_str()[1..]),
                Rule::hue => {
                    keep_case("güe", &pair.as_str())
                },
                Rule::hua => {
                    keep_case("gua", &pair.as_str())
                },
                Rule::inner_h => String::from(&pair.as_str()[1..]),
                Rule::inner_ch => String::from(pair.as_str()),
                _ => String::from(pair.as_str()),
            };
            output.push(chunk);
        }

        Ok(output.join(""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_rule() {
        let input = "hotel HOTEL zanahoria harina chihUahua cacaHuEte";
        let expected = "otel OTEL zanaoria arina chigUagua cacaGüEte";

        let output = h_rule(input).expect("Wrong parser");
        assert_eq!(output, expected);
    }
}
