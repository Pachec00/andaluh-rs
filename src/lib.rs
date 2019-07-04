use std::collections::HashMap;

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
        let map: Option<HashMap<&str, &str>> = None;
        rule!($rule, $input, map, $( $($t)|* => $r ),*)
    }};
    ($rule: expr, $input: expr, $map: expr, $( $($t: pat)|* => $r: expr ),* ) => {{
        let (repl, input) = match $map {
            Some(ref m) => replace_exceptions($input, m),
            None => (vec![], $input.to_string())
        };

        let pairs = AndaluhParser::parse($rule, &input)?;
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

        let mut outstr = output.join("");

        if $map.is_some() {
            outstr = replace_exceptions_back(&outstr, repl);
        }

        Ok(outstr)
    }}
}

fn replace_exceptions(input: &str, m: &HashMap<&str, &str>)
    -> (Vec<(String, String)>, String) {
    let mut map = vec![];
    let mut out = input.to_string();
    for (i, word) in input.split_whitespace().enumerate() {
        let lword = word.to_string().to_lowercase();
        if !m.contains_key(&lword[..]) {
            continue;
        }

        let repl = format!("AND_{}", i);
        out = out.replace(word, &repl);
        map.push((repl, keep_case(m[&lword[..]], word)));
    }

    (map, out)
}

fn replace_exceptions_back(input: &str, map: Vec<(String, String)>) -> String {
    let mut out = input.to_string();
    for (key, value) in map {
        out = out.replace(&key, &value);
    }

    out
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

pub fn h_rule(input: &str) -> Result<String, Error> {
    rule!(Rule::h, &input, Some(&defs::H_RULES_EXCEPT),
        Rule::initial_h | Rule::inner_h => |pair: Pair<Rule>| {
            let s = pair.as_str();
            let h = slice!(s, 0, 1);
            let next = slice!(s, 1);
            keep_case(&next, &h)
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
            circumflex(&prev).to_string() + &keep_case("çç", &x) + &next
        })
}

pub fn ch_rule(input: &str) -> Result<String, Error> {
    rule!(Rule::ch, input,
        Rule::CH => |pair: Pair<Rule>| {
            keep_case("x", pair.as_str())
        })
}

pub fn gj_rule(input: &str) -> Result<String, Error> {
    rule!(Rule::gj, input, Some(&defs::GJ_RULES_EXCEPT),
        Rule::BUE1 => |pair: Pair<Rule>| {
            let groups: Vec<Pair<Rule>> = pair.into_inner().collect();
            let s = groups[0].as_str().to_string();
            let b = groups[1].as_str().to_string();
            let next = groups[2].as_str().to_string();
            s + &keep_case("g", &b) + &next
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
    rule!(Rule::v, input, Some(&defs::V_RULES_EXCEPT),
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
    rule!(Rule::ll, input, Some(&defs::LL_RULES_EXCEPT),
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
            let word_lowercase = word.to_string().to_lowercase();
            let lword = &word_lowercase[..];
            let n = len!(word);
            for pair in pair.into_inner() {
                let chunk = match pair.as_rule() {
                    Rule::ENDING_D => {
                        let exception = defs::WORDEND_D_RULES_EXCEPT.get(lword);
                        if let Some(w) = exception {
                            return keep_case(w, word);
                        }

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
                        let exception = defs::WORDEND_S_RULES_EXCEPT.get(lword);
                        if let Some(w) = exception {
                            return keep_case(w, word);
                        }

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
                        let exception = defs::WORDEND_CONST_RULES_EXCEPT.get(lword);
                        if let Some(w) = exception {
                            return keep_case(w, word);
                        }

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
                        let exception = defs::WORDEND_D_INTERVOWEL_RULES_EXCEPT.get(lword);
                        if let Some(w) = exception {
                            return keep_case(w, word);
                        }

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
                            "idas" => {
                                let vowelb = circumflex(&vowel_b);
                                prefix + &vowel_a + &d + vowelb
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

pub fn word_interaction_rule(input: &str) -> Result<String, Error> {
    rule!(Rule::word_interaction, input,
        Rule::ENDING_L => |pair: Pair<Rule>| {
            let groups: Vec<Pair<Rule>> = pair.into_inner().collect();
            let prefix = groups[0].as_str().to_string();
            let l = groups[1].as_str().to_string();
            let space = groups[2].as_str().to_string();
            let next = groups[3].as_str().to_string();

            prefix + &keep_case("r", &l) + &space + &next
        })
}

pub fn epa(input: &str) -> Result<String, Error> {
    // TODO: escape links
    let rules = [
        h_rule,
        x_rule,
        ch_rule,
        gj_rule,
        v_rule,
        ll_rule,
        l_rule,
        psico_rule,
        vaf_rule,
        word_ending_rule,
        digraph_rule,
        exception_rule,
        word_interaction_rule,
    ];

    let mut output = input.to_string();
    for r in rules.iter() {
        let out = r(&output)?;
        output = out.to_string();
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_rule() {
        let input = "hotel HOTEL zanahoria harina chihUahua cacaHuEte escuchar";
        let expected = "otel OTEL zanaoria arina chigUagua cacaGüEte escuchar";

        let output = h_rule(input).expect("Wrong parser");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_x_rule() {
        let input = "Xilófono axila éxito xenofobia";
        let expected = "Çilófono âççila éççito çenofobia";

        let output = x_rule(input).expect("Wrong parser");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_ch_rule() {
        let input = "Chungo Chachi escuchar";
        let expected = "Xungo Xaxi escuxar";

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
        let input = "Madrid tazas disfraz Colocados tomate tríceps triceps malnacidas";
        let expected = "Madrîh tazâ disfrâh Colocáô tomate trícê triceps malnacidâ";

        let output = word_ending_rule(input).expect("Wrong parser");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_digraph_rule() {
        let input = "asfixian Conmemorar atlántico abstracto perspectiva aerotransporte translado intersticial solsticio superstición aislante";
        let expected = "âffixian Cômmemorar âl-lántico âttrâtto perppêttiva aerotrâpporte trâl-lado intertticial sortticio superttición aîl-lante";

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

    #[test]
    fn test_word_interaction_rule() {
        let input = "el transcurso";
        let expected = "er transcurso";

        let output = word_interaction_rule(input).expect("Wrong parser");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_epa() {
        let test_strings = vec![
            (
            "Todo Xenomorfo dice: [haber], que el Éxito y el éxtasis asfixian, si no eres un xilófono Chungo.",
            "Tó Çenomorfo diçe: [abêh], que el Éççito y el éttaçî âffîççian, çi no erê un çilófono Xungo."
            ),
            (
            "Lleva un Guijarrito el ABuelo, ¡Qué Bueno! ¡para la VERGÜENZA!",
            "Yeba un Giharrito el AGuelo, ¡Qué Gueno! ¡pa la BERGUENÇA!"
            ),
            (
            "VALLA valla, si vas toda de ENVIDIA",
            "BAYA baya, çi bâ toa de EMBIDIA"
            ),
            (
            "Alrededor de la Alpaca había un ALfabeto ALTIVO de valkirias malnacidas",
            "Arrededôh de la Arpaca abía un ARfabeto ARTIBO de barkiriâ mânnaçidâ"
            ),
            (
            "En la Zaragoza y el Japón asexual se Sabía SÉriamente sILBAR con el COxis",
            "En la Çaragoça y er Hapón açêççuâh çe Çabía ÇÉriamente çIRBÂH con er CÔççî"
            ),
            (
            "Transportandonos a la connotación perspicaz del abstracto solsticio de Alaska, el aislante plástico adsorvente asfixió al aMnésico pseudoescritor granadino de constituciones, para ConMemorar broncas adscritas",
            "Trâpportandonô a la cônnotaçión perppicâh del âttrâtto çorttiçio de Alâkka, el aîl-lante pláttico âççorbente âffîççió al ânnéçico çeudoêccritôh granadino de côttituçionê, pa CôMMemorâh broncâ âccritâ"
            ),
            (
            "En la postmodernidad, el transcurso de los transportes y translados en postoperatorios transcienden a la postre unas postillas postpalatales apostilladas se transfieren",
            "En la pômmodênnidá, er trâccurço de lô trâpportê y trâl-láô en pôttoperatoriô trâççienden a la pôttre unâ pôttiyâ pôppalatalê apôttiyâh çe trâffieren"
            ),
            (
            "Venid todos a correr en anorak de visón a Cádiz con actitud y maldad, para escuchar el tríceps de Albéniz tocar ápud con virtud de laúd.",
            "Benîh tôh a corrêh en anorâh de biçón a Cádî con âttitûh y mardá, pa êccuxâh er tríçê de Arbénî tocâh ápû con birtûh de laûh."
            ),
            (
            "Una comida fabada con fado, y sin descuido será casada y amarrada al acolchado roido.",
            "Una comida fabada con fado, y çin dêccuido çerá caçá y amarrá al acorxao roío."
            ),
            (
            "Los SABuesos ChiHuaHUA comían cacaHuETes, FramBuESas y Heno, ¡y HABLAN con hálito de ESPANGLISH!",
            "Lô ÇAGueçô XiGuaGUA comían cacaGuETê, FramBuEÇâ y Eno, ¡y ABLAN con álito de ÊPPANGLÎ!"
            ),
        ];

        for (input, expected) in test_strings {
            let output = epa(input).expect("Wrong parser");
            assert_eq!(output, expected);
        }
    }
}
