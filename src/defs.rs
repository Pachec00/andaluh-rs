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

    pub static ref H_RULES_EXCEPT: HashMap<&'static str, &'static str> = hashmap!{
        "haz" =>  "âh", "hez" =>  "êh", "hoz" =>  "ôh",
        "oh" =>  "ôh",
        "yihad" =>  "yihá",
        "h" =>  "h" // Keep an isolated h as-is
    };

    pub static ref GJ_RULES_EXCEPT: HashMap<&'static str, &'static str> = hashmap!{
        "gin" => "yin", "jazz" => "yâh", "jet" => "yêh"
    };

    pub static ref V_RULES_EXCEPT: HashMap<&'static str, &'static str> = hashmap!{
        "vis" => "bî", "ves" => "bêh"
    };

    pub static ref LL_RULES_EXCEPT: HashMap<&'static str, &'static str> = hashmap!{
        "grill" => "grîh"
    };

    pub static ref WORDEND_D_RULES_EXCEPT: HashMap<&'static str, &'static str> = hashmap!{
        "çed" => "çêh"
    };

    pub static ref WORDEND_S_RULES_EXCEPT: HashMap<&'static str, &'static str> = hashmap!{
        "bies" => "biêh", "bis" => "bîh", "blues" => "blû", "bus" => "bûh",
        "dios" => "diôh", "dos" => "dôh",
        "gas" => "gâh", "gres" => "grêh", "gris" => "grîh",
        "luis" => "luîh",
        "mies" => "miêh", "mus" => "mûh",
        "os" => "ô",
        "pis" => "pîh", "plus" => "plûh", "pus" => "pûh",
        "ras" => "râh", "res" => "rêh",
        "tos" => "tôh", "tres" => "trêh", "tris" => "trîh"
    };

    pub static ref WORDEND_CONST_RULES_EXCEPT: HashMap<&'static str, &'static str> = hashmap!{
        "al" => "al", "cual" => "cuâ", "del" => "del", "dél" => "dél", "el" =>"el", "él" =>"èl", "tal" => "tal", "bil" => "bîl",
        // TODO: uir = huir. Maybe better to add the exceptions on h_rules?
        "por" => "por", "uir" => "huîh",
        // sic, tac
        "çic" => "çic", "tac" => "tac",
        "yak" => "yak",
        "stop" => "êttôh", "bip" => "bip"
    };

    pub static ref WORDEND_D_INTERVOWEL_RULES_EXCEPT: HashMap<&'static str, &'static str> = hashmap!{
        // Ending with -ado
        "fado" => "fado", "cado" => "cado", "nado" => "nado", "priado" => "priado",
        // Ending with -ada
        "fabada" => "fabada", "fabadas" =>"fabadas", "fada" => "fada", "ada" => "ada", "lada" => "lada", "rada" => "rada",
        // Ending with -adas
        "adas" => "adas", "radas" => "radas", "nadas" => "nadas",
        // Ending with -ido
        "aikido" => "aikido", "bûççido" => "bûççido", "çido" => "çido", "cuido" => "cuido", "cupido" => "cupido", "descuido" => "descuido",
        "despido" => "despido", "eido" => "eido", "embido" => "embido", "fido" => "fido", "hido" => "hido", "ido" => "ido", "infido" => "infido",
        "laido" => "laido", "libido" => "libido", "nido" => "nido", "nucleido" => "nucleido", "çonido" => "çonido", "çuido" => "çuido"
    };
}
