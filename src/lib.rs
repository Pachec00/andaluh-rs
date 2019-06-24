use pest::Parser;
use pest_derive::Parser;

use unicode_segmentation::UnicodeSegmentation;

use failure::Error;

macro_rules! chars {
    ($input: expr) => {
        UnicodeSegmentation::graphemes($input, true)
    }
}

macro_rules! slice {
    ($input: expr, $start: expr, $end: expr) => {
        chars!($input)
            .skip($start)
            .take($end - $start)
            .collect::<String>()
    };
    ($input: expr, $start: expr) => {
        chars!($input)
            .skip($start)
            .collect::<String>()
    }
}

#[derive(Parser)]
#[grammar = "andaluh.pest"]
pub struct AndaluhParser;

fn keep_case(input: &str, case: &str) -> String {
    chars!(input).zip(chars!(case))
        .map(|(i, c)| {
            match c.chars().next().unwrap_or('x').is_uppercase() {
                true => i.to_uppercase().to_string(),
                false => i.to_lowercase().to_string(),
            }
        }).collect::<String>()
}

// TODO: Manage RULES_EXCEPT
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

pub fn x_rule(input: &str) -> Result<String, Error> {
        let pairs = AndaluhParser::parse(Rule::x, input)?;
        let mut output: Vec<String> = vec![];

        for pair in pairs {
            let chunk = match pair.as_rule() {
                Rule::initial_x => {
                    let s = &pair.as_str();
                    let next = &s[1..];
                    keep_case("ç", s) + next
                }
                Rule::inner_vowel_x => {
                    let s = pair.as_str();
                    let prev = slice!(s, 0, 1);
                    let x = format!("{0}{0}", slice!(s, 1, 2));
                    let next = slice!(s, 2);
                    prev + &keep_case("çç", &x) + &next
                }
                _ => String::from(pair.as_str()),
            };
            output.push(chunk);
        }

        Ok(output.join(""))
}

pub fn ch_rule(input: &str) -> Result<String, Error> {
        let pairs = AndaluhParser::parse(Rule::ch, input)?;
        let mut output: Vec<String> = vec![];

        for pair in pairs {
            let chunk = match pair.as_rule() {
                Rule::CH => keep_case("x", pair.as_str()),
                _ => String::from(pair.as_str()),
            };
            output.push(chunk);
        }

        Ok(output.join(""))
}

pub fn gj_rule(input: &str) -> Result<String, Error> {
        let pairs = AndaluhParser::parse(Rule::gj, input)?;
        let mut output: Vec<String> = vec![];

        for pair in pairs {
            let chunk = match pair.as_rule() {
                Rule::BUE1 => {
                    let s = pair.as_str();
                    let b = slice!(s, 0, 1);
                    let next = slice!(s, 1);
                    keep_case("g", &b) + &next
                },
                Rule::BUE => {
                    let s = pair.as_str();
                    let prev = slice!(s, 0, 1);
                    let b = slice!(s, 1, 2);
                    let next = slice!(s, 2);
                    prev + &keep_case("g", &b) + &next
                },
                Rule::GJV => {
                    let s = pair.as_str();
                    let gj = slice!(s, 0, 1);
                    let next = slice!(s, 1);
                    keep_case("h", &gj) + &next
                },
                Rule::GUE => {
                    let s = pair.as_str();
                    let g = slice!(s, 0, 1);
                    let next = slice!(s, 2);
                    g + &next
                },
                Rule::GUEd => {
                    let s = pair.as_str();
                    let g = slice!(s, 0, 1);
                    let u = slice!(s, 1, 2);
                    let next = slice!(s, 2);
                    g + &keep_case("u", &u) + &next
                },
                _ => {
                    String::from(pair.as_str())
                },
            };
            output.push(chunk);
        }

        Ok(output.join(""))
}

pub fn v_rule(input: &str) -> Result<String, Error> {
        let pairs = AndaluhParser::parse(Rule::v, input)?;
        let mut output: Vec<String> = vec![];

        for pair in pairs {
            let chunk = match pair.as_rule() {
                Rule::NV => {
                    let s = pair.as_str();
                    keep_case("mb", s)
                },
                Rule::V => {
                    let s = pair.as_str();
                    keep_case("b", s)
                },
                _ => {
                    String::from(pair.as_str())
                },
            };
            output.push(chunk);
        }

        Ok(output.join(""))
}

pub fn ll_rule(input: &str) -> Result<String, Error> {
        let pairs = AndaluhParser::parse(Rule::ll, input)?;
        let mut output: Vec<String> = vec![];

        for pair in pairs {
            let chunk = match pair.as_rule() {
                Rule::LL => {
                    let s = pair.as_str();
                    keep_case("y", s)
                },
                _ => {
                    String::from(pair.as_str())
                },
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

    #[test]
    fn test_x_rule() {
        let input = "Xilófono axila éxito xenofobia";
        let expected = "Çilófono aççila éççito çenofobia";

        let output = x_rule(input).expect("Wrong parser");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_ch_rule() {
        let input = "Chungo Chachi";
        let expected = "Xungo Xaxi";

        let output = ch_rule(input).expect("Wrong parser");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_gj_rule() {
        let input = "Guijarrito ABuelo VERGÜENZA BUEN jamón";
        let expected = "Giharrito AGuelo VERGUENZA GUEN hamón";

        let output = gj_rule(input).expect("Wrong parser");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_v_rule() {
        let input = "envidia valor lleva";
        let expected = "embidia balor lleba";

        let output = v_rule(input).expect("Wrong parser");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_ll_rule() {
        let input = "lleva valla";
        let expected = "yeva vaya";

        let output = ll_rule(input).expect("Wrong parser");
        assert_eq!(output, expected);
    }
}
