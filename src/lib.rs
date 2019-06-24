use pest::Parser;
use pest_derive::Parser;
use pest::iterators::Pair;

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

macro_rules! rule {
    ($rule: expr, $input: expr, $( $t: pat => $r: expr ),* ) => {{
        let pairs = AndaluhParser::parse($rule, $input)?;
        let mut output: Vec<String> = vec![];

        for pair in pairs {
            let chunk = match pair.as_rule() {
                $( $t => {
                    $r(pair)
                } ),*
                _ => {
                    String::from(pair.as_str())
                },
            };
            output.push(chunk);
        }

        Ok(output.join(""))
    }}
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
    rule!(Rule::h, input,
        Rule::initial_h => |pair: Pair<Rule>| {
            String::from(&pair.as_str()[1..])
        },
        Rule::inner_h => |pair: Pair<Rule>| {
            String::from(&pair.as_str()[1..])
        },
        Rule::hue => |pair: Pair<Rule>| {
            keep_case("güe", &pair.as_str())
        },
        Rule::hua => |pair: Pair<Rule>| {
            keep_case("gua", &pair.as_str())
        })
}

pub fn x_rule(input: &str) -> Result<String, Error> {
    rule!(Rule::x, input,
        Rule::initial_x => |pair: Pair<Rule>| {
            let s = &pair.as_str();
            let next = &s[1..];
            keep_case("ç", s) + next
        },
        Rule::inner_vowel_x => |pair: Pair<Rule>| {
            let s = pair.as_str();
            let prev = slice!(s, 0, 1);
            let x = format!("{0}{0}", slice!(s, 1, 2));
            let next = slice!(s, 2);
            prev + &keep_case("çç", &x) + &next
        })
}

pub fn ch_rule(input: &str) -> Result<String, Error> {
    rule!(Rule::ch, input,
        Rule::CH => |pair: Pair<Rule>| {
            keep_case("x", pair.as_str())
        })
}

pub fn gj_rule(input: &str) -> Result<String, Error> {
    rule!(Rule::gj, input,
        Rule::BUE1 => |pair: Pair<Rule>| {
                let s = pair.as_str();
                let b = slice!(s, 0, 1);
                let next = slice!(s, 1);
                keep_case("g", &b) + &next
            },
        Rule::BUE => |pair: Pair<Rule>| {
            let s = pair.as_str();
            let prev = slice!(s, 0, 1);
            let b = slice!(s, 1, 2);
            let next = slice!(s, 2);
            prev + &keep_case("g", &b) + &next
        },
        Rule::GJV => |pair: Pair<Rule>| {
            let s = pair.as_str();
            let gj = slice!(s, 0, 1);
            let next = slice!(s, 1);
            keep_case("h", &gj) + &next
        },
        Rule::GUE => |pair: Pair<Rule>| {
            let s = pair.as_str();
            let g = slice!(s, 0, 1);
            let next = slice!(s, 2);
            g + &next
        },
        Rule::GUEd => |pair: Pair<Rule>| {
            let s = pair.as_str();
            let g = slice!(s, 0, 1);
            let u = slice!(s, 1, 2);
            let next = slice!(s, 2);
            g + &keep_case("u", &u) + &next
        })
}

pub fn v_rule(input: &str) -> Result<String, Error> {
    rule!(Rule::v, input,
        Rule::NV => |pair: Pair<Rule>| {
                    let s = pair.as_str();
                    keep_case("mb", s)
        },
        Rule::V => |pair: Pair<Rule>| {
            let s = pair.as_str();
            keep_case("b", s)
        })
}

pub fn ll_rule(input: &str) -> Result<String, Error> {
    rule!(Rule::ll, input,
        Rule::LL => |pair: Pair<Rule>| {
            let s = pair.as_str();
            keep_case("y", s)
        })
}

pub fn l_rule(input: &str) -> Result<String, Error> {
    rule!(Rule::l, input,
        Rule::L => |pair: Pair<Rule>| {
            let s = pair.as_str();
            keep_case("r", s)
        })
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

    #[test]
    fn test_l_rule() {
        let input = "silbar acolchado";
        let expected = "sirbar acorchado";

        let output = l_rule(input).expect("Wrong parser");
        assert_eq!(output, expected);
    }
}
