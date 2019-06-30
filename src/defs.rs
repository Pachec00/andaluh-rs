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

    pub static ref DIGRAPHS: Vec<&'static str> = vec![
        "bb", "bc", "bç", "bÇ", "bd", "bf", "bg", "bh", "bm", "bn", "bp", "bq", "bt", "bx", "by", "cb", "cc",
        "cç", "cÇ", "cd", "cf", "cg", "ch", "cm", "cn", "cp", "cq", "ct", "cx", "cy",
        "db", "dc", "dç", "dÇ", "dd", "df", "dg", "dh", "dl", "dm", "dn", "dp", "dq", "dt", "dx", "dy",
        "fb", "fc", "fç", "fÇ", "fd", "ff", "fg", "fh", "fm", "fn", "fp", "fq", "ft", "fx", "fy",
        "gb", "gc", "gç", "gÇ", "gd", "gf", "gg", "gh", "gm", "gn", "gp", "gq", "gt", "gx", "gy",
        "jb", "jc", "jç", "jÇ", "jd", "jf", "jg", "jh", "jl", "jm", "jn", "jp", "jq", "jr", "jt", "jx", "jy",
        "lb", "lc", "lç", "lÇ", "ld", "lf", "lg", "lh", "ll", "lm", "ln", "lp", "lq", "lr", "lt", "lx", "ly",
        "mm", "mn",
        "nm", "nn",
        "pb", "pc", "pç", "pÇ", "pd", "pf", "pg", "ph", "pm", "pn", "pp", "pq", "pt", "px", "py",
        "rn",
        "sb", "sc", "sç", "sÇ", "sd", "sf", "sg", "sh", "sk", "sl", "sm", "sn", "sñ", "sp", "sq", "sr", "st", "sx", "sy",
        "tb", "tc", "tç", "tÇ", "td", "tf", "tg", "th", "tl", "tm", "tn", "tp", "tq", "tt", "tx", "ty",
        "xb", "xc", "xç", "xÇ", "xd", "xf", "xg", "xh", "xl", "xm", "xn", "xp", "xq", "xr", "xt", "xx", "xy",
        "zb", "zc", "zç", "zÇ", "zd", "zf", "zg", "zh", "zl", "zm", "zn", "zp", "zq", "zr", "zt", "zx", "zy"
    ];
}
