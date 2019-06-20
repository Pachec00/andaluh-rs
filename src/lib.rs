use pest::Parser;
use pest_derive::Parser;

use failure::Error;

#[derive(Parser)]
#[grammar = "andaluh.pest"]
pub struct AndaluhParser;

pub fn epa() {
    // let pairs = AndaluhParser::parse(Rule::ident, "a1 b2").unwrap_or_else(|e| panic!("{}", e));

    // Because ident_list is silent, the iterator will contain idents
    // for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
    //     println!("Rule:    {:?}", pair.as_rule());
    //     println!("Span:    {:?}", pair.as_span());
    //     println!("Text:    {}", pair.as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
    //     for inner_pair in pair.into_inner() {
    //         match inner_pair.as_rule() {
    //             Rule::alpha => println!("Letter:  {}", inner_pair.as_str()),
    //             Rule::digit => println!("Digit:   {}", inner_pair.as_str()),
    //             _ => unreachable!()
    //         };
    //     }
    // }
}

pub fn h_rule(input: &str) -> Result<String, Error> {
        let pairs = AndaluhParser::parse(Rule::h, input)?;
        let mut output: Vec<&str> = vec![];

        for pair in pairs {
            let chunk = match pair.as_rule() {
                Rule::initial_h => &pair.as_str()[1..],
                // TODO: mantain the CASE
                Rule::hue => "gûe",
                Rule::hua => "gua",
                Rule::inner_h => &pair.as_str()[1..],
                Rule::inner_ch => &pair.as_str(),
                _ => &pair.as_str(),
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
        let input = "hotel HOTEL zanahoria harina chihuahua cacahuete";
        let expected = "otel OTEL zanaoria arina chiguagua cacagûete";

        let output = h_rule(input).expect("Wrong parser");
        assert_eq!(output, expected);
    }
}
