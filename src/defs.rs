use lazy_static::lazy_static;
use maplit::hashmap;
use std::collections::HashMap;

lazy_static! {
    pub static ref WORD_ENDING_D_UNSTRESS: HashMap<&'static str, &'static str> = hashmap!{
        "a" => "â", "A" => "Â", "á" => "â", "Á" => "Â",
        "e" => "ê", "E" => "Ê", "é" => "ê", "É" => "Ê",
        "i" => "î", "I" => "Î", "í" => "î", "Í" => "Î",
        "o" => "ô", "O" => "Ô", "ó" => "ô", "Ó" => "Ô",
        "u" => "û", "U" => "Û", "ú" => "û", "Ú" => "Û"
    };
    pub static ref WORD_ENDING_D_STRESS: HashMap<&'static str, &'static str> = hashmap!{
        "a" => "á", "A" => "Á", "á" => "á", "Á" => "Á",
        "e" => "é", "E" => "É", "é" => "é", "É" => "É",
        "i" => "î", "I" => "Î", "í" => "î", "Í" => "Î",
        "o" => "ô", "O" => "Ô", "ó" => "ô", "Ó" => "Ô",
        "u" => "û", "U" => "Û", "ú" => "û", "Ú" => "Û"
    };
    pub static ref WORD_ENDING_S: HashMap<&'static str, &'static str> = hashmap!{
        "a" => "â", "A" => "Â", "á" => "â", "Á" => "Â",
        "e" => "ê", "E" => "Ê", "é" => "ê", "É" => "Ê",
        "i" => "î", "I" => "Î", "í" => "î", "Í" => "Î",
        "o" => "ô", "O" => "Ô", "ó" => "ô", "Ó" => "Ô",
        "u" => "û", "U" => "Û", "ú" => "û", "Ú" => "Û"
    };
    pub static ref WORD_ENDING_CONS: HashMap<&'static str, &'static str> = hashmap!{
        "a" => "â", "A" => "Â", "á" => "â", "Á" => "Â",
        "e" => "ê", "E" => "Ê", "é" => "ê", "É" => "Ê",
        "i" => "î", "I" => "Î", "í" => "î", "Í" => "Î",
        "o" => "ô", "O" => "Ô", "ó" => "ô", "Ó" => "Ô",
        "u" => "û", "U" => "Û", "ú" => "û", "Ú" => "Û"
    };
}
