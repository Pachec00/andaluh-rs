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

    pub static ref ENDING_RULES_EXCEPTION: HashMap<&'static str, &'static str> = hashmap!{
        // Exceptions to digraph rules with nm
        "biêmmandao" => "bienmandao", "biêmmeçabe" => "bienmeçabe", "buêmmoço" => "buenmoço", "çiêmmiléçima" => "çienmiléçima", "çiêmmiléçimo" => "çienmiléçimo", "çiêmmilímetro" => "çienmilímetro", "çiêmmiyonéçima" => "çienmiyonéçima", "çiêmmiyonéçimo" => "çienmiyonéçimo", "çiêmmirmiyonéçima" => "çienmirmiyonéçima", "çiêmmirmiyonéçimo" => "çienmirmiyonéçimo",
        // Exceptions to l rules
        "marrotadôh" => "mârrotadôh", "marrotâh" => "mârrotâh", "mirrayâ" => "mîrrayâ",
        // Exceptions to psico pseudo rules
        "herôççiquiatría" => "heroçiquiatría", "herôççiquiátrico" => "heroçiquiátrico", "farmacôççiquiatría" => "farmacoçiquiatría", "metempçícoçî" => "metemçícoçî", "necróçico" => "necróççico", "pampçiquîmmo" => "pamçiquîmmo",
        // Other exceptions
        "antîççerôttármico" => "antiçerôttármico", "eclampçia" => "eclampçia", "pôttoperatorio" => "pôççoperatorio", "çáccrito" => "çánccrito", "manbîh" => "mambîh", "cômmelináçeo" => "commelináçeo", "dîmmneçia" => "dînneçia", "todo" => "tó", "todô" => "tôh", "toda" => "toa", "todâ" => "toâ",
        // Other exceptions monosyllables
        "as" => "âh", "clown" => "claun", "crack" => "crâh", "down" => "daun", "es" => "êh", "ex" => "êh", "ir" => "îh", "miss" => "mîh", "muy" => "mu", "ôff" => "off", "os" => "ô", "para" => "pa", "ring" => "rin", "rock" => "rôh", "spray" => "êppray", "sprint" => "êpprín", "wau" => "guau"
    };
}
