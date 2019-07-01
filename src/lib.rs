use pest::Parser;
use pest_derive::Parser;
use pest::iterators::Pair;

use unicode_segmentation::UnicodeSegmentation;

use failure::Error;

mod defs;

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

macro_rules! len {
    ($input: expr) => {
        chars!($input).count()
    }
}

macro_rules! rule {
    ($rule: expr, $input: expr, $( $($t: pat)|* => $r: expr ),* ) => {{
        let pairs = AndaluhParser::parse($rule, $input)?;
        let mut output: Vec<String> = vec![];

        for pair in pairs {
            let chunk = match pair.as_rule() {
                $( $($t)|* => {
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

fn circumflex<'a>(vowel: &'a str) -> &'a str {
    match vowel {
        "a" => "â",
        "e" => "ê",
        "i" => "î",
        "o" => "ô",
        "u" => "û",
        "A" => "Â",
        "E" => "Ê",
        "I" => "Î",
        "O" => "Ô",
        "U" => "Û",
        _ => vowel
    }
}

fn tilde<'a>(vowel: &'a str) -> &'a str {
    match vowel {
        "a" => "á",
        "e" => "é",
        "i" => "í",
        "o" => "ó",
        "u" => "ú",
        "A" => "Á",
        "E" => "É",
        "I" => "Í",
        "O" => "Ó",
        "U" => "Ú",
        _ => vowel
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
    rule!(Rule::h, input,
        Rule::initial_h | Rule::inner_h => |pair: Pair<Rule>| {
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

pub fn psico_rule(input: &str) -> Result<String, Error> {
    rule!(Rule::psico, input,
        Rule::PSIC | Rule::PSEUD => |pair: Pair<Rule>| {
            let s = pair.as_str();
            slice!(s, 1)
        })
}

pub fn vaf_rule(input: &str) -> Result<String, Error> {
    rule!(Rule::vaf, input,
        Rule::ZSv | Rule::Cv => |pair: Pair<Rule>| {
            let s = pair.as_str();
            let next = slice!(s, 1);
            keep_case("ç", s) + &next
        })
}

pub fn word_ending_rule(input: &str) -> Result<String, Error> {
    rule!(Rule::word_ending, input,
        Rule::ENDING_WORD => |pair: Pair<Rule>| {
            let mut output: Vec<String> = vec![];
            let word = pair.as_str();
            let n = len!(word);
            for pair in pair.into_inner() {
                let chunk = match pair.as_rule() {
                    Rule::ENDING_D => {
                        let prefix = slice!(word, 0, n - 2);
                        let vowel = slice!(word, n - 2, n - 1);
                        let d = slice!(word, n - 1);

                        let pl = prefix.to_lowercase();
                        if ["á", "é", "í", "ó", "ú"].iter().any(|x| pl.contains(x)) {
                            return prefix + defs::WORD_ENDING_D_UNSTRESS[&vowel[..]];
                        }

                        match &vowel.to_lowercase()[..] {
                            "a" | "e" | "á" | "é" => {
                                prefix + defs::WORD_ENDING_D_STRESS[&vowel[..]]
                            }
                            _ => {
                                let part1 = defs::WORD_ENDING_D_STRESS[&vowel[..]];
                                prefix + part1 + &keep_case("h", &d)
                            }
                        }
                    },
                    Rule::ENDING_S => {
                        let prefix = slice!(word, 0, n - 2);
                        let vowel = slice!(word, n - 2, n - 1);
                        let s = slice!(word, n - 1);

                        match &vowel.to_lowercase()[..] {
                            "á" | "é" | "í" | "ó" | "ú" => {
                                prefix + defs::WORD_ENDING_S[&vowel[..]] + &keep_case("h", &s)
                            }
                            _ => {
                                prefix + defs::WORD_ENDING_S[&vowel[..]]
                            }
                        }
                    },
                    Rule::ENDING_CONS => {
                        let prefix = slice!(word, 0, n - 2);
                        let vowel = slice!(word, n - 2, n - 1);
                        let c = slice!(word, n - 1);

                        let pl = prefix.to_lowercase();
                        if ["á", "é", "í", "ó", "ú"].iter().any(|x| pl.contains(x)) {
                            return prefix + defs::WORD_ENDING_CONS[&vowel[..]];
                        }

                        prefix + defs::WORD_ENDING_CONS[&vowel[..]] + &keep_case("h", &c)
                    },
                    Rule::ENDING_PS => {
                        let prefix = slice!(word, 0, n - 3);
                        let e = slice!(word, n - 3, n - 2);

                        let pl = prefix.to_lowercase();
                        if ["á", "é", "í", "ó", "ú"].iter().any(|x| pl.contains(x)) {
                            return prefix + circumflex(&e)
                        }

                        word.to_string()
                    },
                    Rule::INTER_D => {
                        let last = slice!(word, n - 1);
                        let (n, last_s) = match &last[..] {
                            "s" | "S" => (n - 1, true),
                            _ => (n, false)
                        };

                        let prefix = slice!(word, 0, n - 3);
                        let vowel_a = slice!(word, n - 3, n - 2);
                        let d = slice!(word, n - 2, n - 1);
                        let vowel_b = slice!(word, n - 1, n);

                        let mut suffix = String::new() + &vowel_a + &d + &vowel_b;
                        if last_s {
                            suffix += &last;
                        }

                        let pl = prefix.to_lowercase();
                        if ["á", "é", "í", "ó", "ú"].iter().any(|x| pl.contains(x)) {
                            return word.to_string();
                        }

                        match &suffix.to_lowercase()[..] {
                            "ada" => {
                                prefix + &keep_case("á", &vowel_b)
                            }
                            "adas" => {
                                let vowel = circumflex(&vowel_a);
                                let sfx = vowel.to_string() + "h";
                                prefix + &keep_case(&sfx, &slice!(&suffix[..], 0, 2))
                            }
                            "ado" => {
                                prefix + &vowel_a + &vowel_b
                            }
                            "ados" | "idos" | "ídos" => {
                                let vowelb = circumflex(&vowel_b);
                                let vowela = tilde(&vowel_a);
                                prefix + vowela + vowelb
                            }
                            "ido" | "ído" => {
                                prefix + &keep_case("í", &vowel_a[..]) + &vowel_b
                            }
                            _ => word.to_string()
                        }
                    },
                    _ => {
                        "".to_string()
                    },
                };
                output.push(chunk);
            }
            output.join("")
        })
}

pub fn digraph_rule(input: &str) -> Result<String, Error> {
    rule!(Rule::digraph, input,
        Rule::DIGRAPH_EXP_LSTRST => |pair: Pair<Rule>| {
            let groups: Vec<Pair<Rule>> = pair.into_inner().collect();
            let vowel = groups[0].as_str().to_string();
            let lr_char = groups[1].as_str().to_string();
            let t_char = groups[3].as_str().to_string();

            let lr_repl = match &lr_char[..] {
                "l"|"L" => keep_case("r", &lr_char),
                _ => lr_char
            };

            vowel + &lr_repl + &format!("{0}{0}", t_char)
        },
        Rule::DIGRAPH_EXP_TRANS => |pair: Pair<Rule>| {
            let groups: Vec<Pair<Rule>> = pair.into_inner().collect();
            let init = groups[0].as_str().to_string();
            let vowel = groups[1].as_str().to_string();
            let cons = groups[3].as_str().to_string();

            match &cons[..] {
                "L"|"l" => init + circumflex(&vowel) + &format!("{0}-{0}", cons),
                _ => init + circumflex(&vowel) + &format!("{0}{0}", cons)
            }
        },
        Rule::DIGRAPH_EXP_BDNR => |pair: Pair<Rule>| {
            let s = pair.as_str();
            let vowel = slice!(s, 0, 1);
            let bdnr = slice!(s, 1, 2);
            let digraph = slice!(s, 3);

            match &bdnr[..] {
                "R" | "r" => {
                    vowel + &bdnr + &format!("{0}{0}", digraph)
                }
                _ => {
                    circumflex(&vowel).to_string() + &format!("{0}{0}", digraph)
                }
            }
        },
        Rule::DIGRAPH_EXP_L => |pair: Pair<Rule>| {
            let s = pair.as_str();
            let vowel = slice!(s, 0, 1);
            let digraph = slice!(s, 2);

            circumflex(&vowel).to_string() + &format!("{0}-{0}", digraph)
        },
        Rule::GEN_DIGRAPH => |pair: Pair<Rule>| {
            let s = pair.as_str();
            let vowel = slice!(s, 0, 1);
            let digraph = slice!(s, 2);

            circumflex(&vowel).to_string() + &format!("{0}{0}", digraph)
        })
}

pub fn exception_rule(input: &str) -> Result<String, Error> {
    rule!(Rule::exception, input,
        Rule::word => |pair: Pair<Rule>| {
            let s = pair.as_str();
            let out = defs::ENDING_RULES_EXCEPTION
                .get(&s.to_lowercase()[..])
                .unwrap_or(&s);

            keep_case(out, s)
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

    #[test]
    fn test_psico_rule() {
        let input = "psicologo pseudoescritor";
        let expected = "sicologo seudoescritor";

        let output = psico_rule(input).expect("Wrong parser");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_vaf_rule() {
        let input = "Zaragoza solsticio";
        let expected = "Çaragoça çolstiçio";

        let output = vaf_rule(input).expect("Wrong parser");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_word_ending_rule() {
        let input = "Madrid tazas disfraz Colocados tomate tríceps triceps";
        let expected = "Madrîh tazâ disfrâh Colocáô tomate trícê triceps";

        let output = word_ending_rule(input).expect("Wrong parser");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_digraph_rule() {
        let input = "asfixian Conmemorar atlántico abstracto perspectiva aerotransporte translado intersticial solsticio superstición";
        let expected = "âffixian Cômmemorar âl-lántico âttrâtto perppêttiva aerotrâpporte trâl-lado intertticial sortticio superttición";

        let output = digraph_rule(input).expect("Wrong parser");
        assert_eq!(output, expected);
    }
        #[test]
    fn test_exception_rule() {
        let input = "tomate biêmmandao TuRuRú crack";
        let expected = "tomate bienmandao TuRuRú crâh";

        let output = exception_rule(input).expect("Wrong parser");
        assert_eq!(output, expected);
    }
}
