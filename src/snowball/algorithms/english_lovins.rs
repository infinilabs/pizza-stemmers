//! Generated by Snowball 2.2.0 - https://snowballstem.org/

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
use super::super::among::Among;
use super::super::env::SnowballEnv;

static A_0: &'static [Among<Context>; 9] = &[
    Among("d", -1, -1, None),
    Among("f", -1, -1, None),
    Among("ph", -1, -1, None),
    Among("th", -1, -1, None),
    Among("l", -1, -1, None),
    Among("er", -1, -1, None),
    Among("or", -1, -1, None),
    Among("es", -1, -1, None),
    Among("t", -1, -1, None),
];

static A_1: &'static [Among<Context>; 294] = &[
    Among("s'", -1, 1, Some(&r_A)),
    Among("a", -1, 1, Some(&r_A)),
    Among("ia", 1, 1, Some(&r_A)),
    Among("ata", 1, 1, Some(&r_A)),
    Among("ic", -1, 1, Some(&r_A)),
    Among("aic", 4, 1, Some(&r_A)),
    Among("allic", 4, 1, Some(&r_BB)),
    Among("aric", 4, 1, Some(&r_A)),
    Among("atic", 4, 1, Some(&r_B)),
    Among("itic", 4, 1, Some(&r_H)),
    Among("antic", 4, 1, Some(&r_C)),
    Among("istic", 4, 1, Some(&r_A)),
    Among("alistic", 11, 1, Some(&r_B)),
    Among("aristic", 11, 1, Some(&r_A)),
    Among("ivistic", 11, 1, Some(&r_A)),
    Among("ed", -1, 1, Some(&r_E)),
    Among("anced", 15, 1, Some(&r_B)),
    Among("enced", 15, 1, Some(&r_A)),
    Among("ished", 15, 1, Some(&r_A)),
    Among("ied", 15, 1, Some(&r_A)),
    Among("ened", 15, 1, Some(&r_E)),
    Among("ioned", 15, 1, Some(&r_A)),
    Among("ated", 15, 1, Some(&r_I)),
    Among("ented", 15, 1, Some(&r_C)),
    Among("ized", 15, 1, Some(&r_F)),
    Among("arized", 24, 1, Some(&r_A)),
    Among("oid", -1, 1, Some(&r_A)),
    Among("aroid", 26, 1, Some(&r_A)),
    Among("hood", -1, 1, Some(&r_A)),
    Among("ehood", 28, 1, Some(&r_A)),
    Among("ihood", 28, 1, Some(&r_A)),
    Among("elihood", 30, 1, Some(&r_E)),
    Among("ward", -1, 1, Some(&r_A)),
    Among("e", -1, 1, Some(&r_A)),
    Among("ae", 33, 1, Some(&r_A)),
    Among("ance", 33, 1, Some(&r_B)),
    Among("icance", 35, 1, Some(&r_A)),
    Among("ence", 33, 1, Some(&r_A)),
    Among("ide", 33, 1, Some(&r_L)),
    Among("icide", 38, 1, Some(&r_A)),
    Among("otide", 38, 1, Some(&r_A)),
    Among("age", 33, 1, Some(&r_B)),
    Among("able", 33, 1, Some(&r_A)),
    Among("atable", 42, 1, Some(&r_A)),
    Among("izable", 42, 1, Some(&r_E)),
    Among("arizable", 44, 1, Some(&r_A)),
    Among("ible", 33, 1, Some(&r_A)),
    Among("encible", 46, 1, Some(&r_A)),
    Among("ene", 33, 1, Some(&r_E)),
    Among("ine", 33, 1, Some(&r_M)),
    Among("idine", 49, 1, Some(&r_I)),
    Among("one", 33, 1, Some(&r_R)),
    Among("ature", 33, 1, Some(&r_E)),
    Among("eature", 52, 1, Some(&r_Z)),
    Among("ese", 33, 1, Some(&r_A)),
    Among("wise", 33, 1, Some(&r_A)),
    Among("ate", 33, 1, Some(&r_A)),
    Among("entiate", 56, 1, Some(&r_A)),
    Among("inate", 56, 1, Some(&r_A)),
    Among("ionate", 56, 1, Some(&r_D)),
    Among("ite", 33, 1, Some(&r_AA)),
    Among("ive", 33, 1, Some(&r_A)),
    Among("ative", 61, 1, Some(&r_A)),
    Among("ize", 33, 1, Some(&r_F)),
    Among("alize", 63, 1, Some(&r_A)),
    Among("icalize", 64, 1, Some(&r_A)),
    Among("ialize", 64, 1, Some(&r_A)),
    Among("entialize", 66, 1, Some(&r_A)),
    Among("ionalize", 64, 1, Some(&r_A)),
    Among("arize", 63, 1, Some(&r_A)),
    Among("ing", -1, 1, Some(&r_N)),
    Among("ancing", 70, 1, Some(&r_B)),
    Among("encing", 70, 1, Some(&r_A)),
    Among("aging", 70, 1, Some(&r_B)),
    Among("ening", 70, 1, Some(&r_E)),
    Among("ioning", 70, 1, Some(&r_A)),
    Among("ating", 70, 1, Some(&r_I)),
    Among("enting", 70, 1, Some(&r_C)),
    Among("ying", 70, 1, Some(&r_B)),
    Among("izing", 70, 1, Some(&r_F)),
    Among("arizing", 79, 1, Some(&r_A)),
    Among("ish", -1, 1, Some(&r_C)),
    Among("yish", 81, 1, Some(&r_A)),
    Among("i", -1, 1, Some(&r_A)),
    Among("al", -1, 1, Some(&r_BB)),
    Among("ical", 84, 1, Some(&r_A)),
    Among("aical", 85, 1, Some(&r_A)),
    Among("istical", 85, 1, Some(&r_A)),
    Among("oidal", 84, 1, Some(&r_A)),
    Among("eal", 84, 1, Some(&r_Y)),
    Among("ial", 84, 1, Some(&r_A)),
    Among("ancial", 90, 1, Some(&r_A)),
    Among("arial", 90, 1, Some(&r_A)),
    Among("ential", 90, 1, Some(&r_A)),
    Among("ional", 84, 1, Some(&r_A)),
    Among("ational", 94, 1, Some(&r_B)),
    Among("izational", 95, 1, Some(&r_A)),
    Among("ental", 84, 1, Some(&r_A)),
    Among("ful", -1, 1, Some(&r_A)),
    Among("eful", 98, 1, Some(&r_A)),
    Among("iful", 98, 1, Some(&r_A)),
    Among("yl", -1, 1, Some(&r_R)),
    Among("ism", -1, 1, Some(&r_B)),
    Among("icism", 102, 1, Some(&r_A)),
    Among("oidism", 102, 1, Some(&r_A)),
    Among("alism", 102, 1, Some(&r_B)),
    Among("icalism", 105, 1, Some(&r_A)),
    Among("ionalism", 105, 1, Some(&r_A)),
    Among("inism", 102, 1, Some(&r_J)),
    Among("ativism", 102, 1, Some(&r_A)),
    Among("um", -1, 1, Some(&r_U)),
    Among("ium", 110, 1, Some(&r_A)),
    Among("ian", -1, 1, Some(&r_A)),
    Among("ician", 112, 1, Some(&r_A)),
    Among("en", -1, 1, Some(&r_F)),
    Among("ogen", 114, 1, Some(&r_A)),
    Among("on", -1, 1, Some(&r_S)),
    Among("ion", 116, 1, Some(&r_Q)),
    Among("ation", 117, 1, Some(&r_B)),
    Among("ication", 118, 1, Some(&r_G)),
    Among("entiation", 118, 1, Some(&r_A)),
    Among("ination", 118, 1, Some(&r_A)),
    Among("isation", 118, 1, Some(&r_A)),
    Among("arisation", 122, 1, Some(&r_A)),
    Among("entation", 118, 1, Some(&r_A)),
    Among("ization", 118, 1, Some(&r_F)),
    Among("arization", 125, 1, Some(&r_A)),
    Among("action", 117, 1, Some(&r_G)),
    Among("o", -1, 1, Some(&r_A)),
    Among("ar", -1, 1, Some(&r_X)),
    Among("ear", 129, 1, Some(&r_Y)),
    Among("ier", -1, 1, Some(&r_A)),
    Among("ariser", -1, 1, Some(&r_A)),
    Among("izer", -1, 1, Some(&r_F)),
    Among("arizer", 133, 1, Some(&r_A)),
    Among("or", -1, 1, Some(&r_T)),
    Among("ator", 135, 1, Some(&r_A)),
    Among("s", -1, 1, Some(&r_W)),
    Among("'s", 137, 1, Some(&r_A)),
    Among("as", 137, 1, Some(&r_B)),
    Among("ics", 137, 1, Some(&r_A)),
    Among("istics", 140, 1, Some(&r_A)),
    Among("es", 137, 1, Some(&r_E)),
    Among("ances", 142, 1, Some(&r_B)),
    Among("ences", 142, 1, Some(&r_A)),
    Among("ides", 142, 1, Some(&r_L)),
    Among("oides", 145, 1, Some(&r_A)),
    Among("ages", 142, 1, Some(&r_B)),
    Among("ies", 142, 1, Some(&r_P)),
    Among("acies", 148, 1, Some(&r_A)),
    Among("ancies", 148, 1, Some(&r_A)),
    Among("encies", 148, 1, Some(&r_A)),
    Among("aries", 148, 1, Some(&r_A)),
    Among("ities", 148, 1, Some(&r_A)),
    Among("alities", 153, 1, Some(&r_A)),
    Among("ivities", 153, 1, Some(&r_A)),
    Among("ines", 142, 1, Some(&r_M)),
    Among("nesses", 142, 1, Some(&r_A)),
    Among("ates", 142, 1, Some(&r_A)),
    Among("atives", 142, 1, Some(&r_A)),
    Among("ings", 137, 1, Some(&r_N)),
    Among("is", 137, 1, Some(&r_A)),
    Among("als", 137, 1, Some(&r_BB)),
    Among("ials", 162, 1, Some(&r_A)),
    Among("entials", 163, 1, Some(&r_A)),
    Among("ionals", 162, 1, Some(&r_A)),
    Among("isms", 137, 1, Some(&r_B)),
    Among("ians", 137, 1, Some(&r_A)),
    Among("icians", 167, 1, Some(&r_A)),
    Among("ions", 137, 1, Some(&r_B)),
    Among("ations", 169, 1, Some(&r_B)),
    Among("arisations", 170, 1, Some(&r_A)),
    Among("entations", 170, 1, Some(&r_A)),
    Among("izations", 170, 1, Some(&r_A)),
    Among("arizations", 173, 1, Some(&r_A)),
    Among("ars", 137, 1, Some(&r_O)),
    Among("iers", 137, 1, Some(&r_A)),
    Among("izers", 137, 1, Some(&r_F)),
    Among("ators", 137, 1, Some(&r_A)),
    Among("less", 137, 1, Some(&r_A)),
    Among("eless", 179, 1, Some(&r_A)),
    Among("ness", 137, 1, Some(&r_A)),
    Among("eness", 181, 1, Some(&r_E)),
    Among("ableness", 182, 1, Some(&r_A)),
    Among("eableness", 183, 1, Some(&r_E)),
    Among("ibleness", 182, 1, Some(&r_A)),
    Among("ateness", 182, 1, Some(&r_A)),
    Among("iteness", 182, 1, Some(&r_A)),
    Among("iveness", 182, 1, Some(&r_A)),
    Among("ativeness", 188, 1, Some(&r_A)),
    Among("ingness", 181, 1, Some(&r_A)),
    Among("ishness", 181, 1, Some(&r_A)),
    Among("iness", 181, 1, Some(&r_A)),
    Among("ariness", 192, 1, Some(&r_E)),
    Among("alness", 181, 1, Some(&r_A)),
    Among("icalness", 194, 1, Some(&r_A)),
    Among("antialness", 194, 1, Some(&r_A)),
    Among("entialness", 194, 1, Some(&r_A)),
    Among("ionalness", 194, 1, Some(&r_A)),
    Among("fulness", 181, 1, Some(&r_A)),
    Among("lessness", 181, 1, Some(&r_A)),
    Among("ousness", 181, 1, Some(&r_A)),
    Among("eousness", 201, 1, Some(&r_A)),
    Among("iousness", 201, 1, Some(&r_A)),
    Among("itousness", 201, 1, Some(&r_A)),
    Among("entness", 181, 1, Some(&r_A)),
    Among("ants", 137, 1, Some(&r_B)),
    Among("ists", 137, 1, Some(&r_A)),
    Among("icists", 207, 1, Some(&r_A)),
    Among("us", 137, 1, Some(&r_V)),
    Among("ous", 209, 1, Some(&r_A)),
    Among("eous", 210, 1, Some(&r_A)),
    Among("aceous", 211, 1, Some(&r_A)),
    Among("antaneous", 211, 1, Some(&r_A)),
    Among("ious", 210, 1, Some(&r_A)),
    Among("acious", 214, 1, Some(&r_B)),
    Among("itous", 210, 1, Some(&r_A)),
    Among("ant", -1, 1, Some(&r_B)),
    Among("icant", 217, 1, Some(&r_A)),
    Among("ent", -1, 1, Some(&r_C)),
    Among("ement", 219, 1, Some(&r_A)),
    Among("izement", 220, 1, Some(&r_A)),
    Among("ist", -1, 1, Some(&r_A)),
    Among("icist", 222, 1, Some(&r_A)),
    Among("alist", 222, 1, Some(&r_A)),
    Among("icalist", 224, 1, Some(&r_A)),
    Among("ialist", 224, 1, Some(&r_A)),
    Among("ionist", 222, 1, Some(&r_A)),
    Among("entist", 222, 1, Some(&r_A)),
    Among("y", -1, 1, Some(&r_B)),
    Among("acy", 229, 1, Some(&r_A)),
    Among("ancy", 229, 1, Some(&r_B)),
    Among("ency", 229, 1, Some(&r_A)),
    Among("ly", 229, 1, Some(&r_B)),
    Among("ealy", 233, 1, Some(&r_Y)),
    Among("ably", 233, 1, Some(&r_A)),
    Among("ibly", 233, 1, Some(&r_A)),
    Among("edly", 233, 1, Some(&r_E)),
    Among("iedly", 237, 1, Some(&r_A)),
    Among("ely", 233, 1, Some(&r_E)),
    Among("ately", 239, 1, Some(&r_A)),
    Among("ively", 239, 1, Some(&r_A)),
    Among("atively", 241, 1, Some(&r_A)),
    Among("ingly", 233, 1, Some(&r_B)),
    Among("atingly", 243, 1, Some(&r_A)),
    Among("ily", 233, 1, Some(&r_A)),
    Among("lily", 245, 1, Some(&r_A)),
    Among("arily", 245, 1, Some(&r_A)),
    Among("ally", 233, 1, Some(&r_B)),
    Among("ically", 248, 1, Some(&r_A)),
    Among("aically", 249, 1, Some(&r_A)),
    Among("allically", 249, 1, Some(&r_C)),
    Among("istically", 249, 1, Some(&r_A)),
    Among("alistically", 252, 1, Some(&r_B)),
    Among("oidally", 248, 1, Some(&r_A)),
    Among("ially", 248, 1, Some(&r_A)),
    Among("entially", 255, 1, Some(&r_A)),
    Among("ionally", 248, 1, Some(&r_A)),
    Among("ationally", 257, 1, Some(&r_B)),
    Among("izationally", 258, 1, Some(&r_B)),
    Among("entally", 248, 1, Some(&r_A)),
    Among("fully", 233, 1, Some(&r_A)),
    Among("efully", 261, 1, Some(&r_A)),
    Among("ifully", 261, 1, Some(&r_A)),
    Among("enly", 233, 1, Some(&r_E)),
    Among("arly", 233, 1, Some(&r_K)),
    Among("early", 265, 1, Some(&r_Y)),
    Among("lessly", 233, 1, Some(&r_A)),
    Among("ously", 233, 1, Some(&r_A)),
    Among("eously", 268, 1, Some(&r_A)),
    Among("iously", 268, 1, Some(&r_A)),
    Among("ently", 233, 1, Some(&r_A)),
    Among("ary", 229, 1, Some(&r_F)),
    Among("ery", 229, 1, Some(&r_E)),
    Among("icianry", 229, 1, Some(&r_A)),
    Among("atory", 229, 1, Some(&r_A)),
    Among("ity", 229, 1, Some(&r_A)),
    Among("acity", 276, 1, Some(&r_A)),
    Among("icity", 276, 1, Some(&r_A)),
    Among("eity", 276, 1, Some(&r_A)),
    Among("ality", 276, 1, Some(&r_A)),
    Among("icality", 280, 1, Some(&r_A)),
    Among("iality", 280, 1, Some(&r_A)),
    Among("antiality", 282, 1, Some(&r_A)),
    Among("entiality", 282, 1, Some(&r_A)),
    Among("ionality", 280, 1, Some(&r_A)),
    Among("elity", 276, 1, Some(&r_A)),
    Among("ability", 276, 1, Some(&r_A)),
    Among("izability", 287, 1, Some(&r_A)),
    Among("arizability", 288, 1, Some(&r_A)),
    Among("ibility", 276, 1, Some(&r_A)),
    Among("inity", 276, 1, Some(&r_CC)),
    Among("arity", 276, 1, Some(&r_B)),
    Among("ivity", 276, 1, Some(&r_A)),
];

static A_2: &'static [Among<Context>; 10] = &[
    Among("bb", -1, -1, None),
    Among("dd", -1, -1, None),
    Among("gg", -1, -1, None),
    Among("ll", -1, -1, None),
    Among("mm", -1, -1, None),
    Among("nn", -1, -1, None),
    Among("pp", -1, -1, None),
    Among("rr", -1, -1, None),
    Among("ss", -1, -1, None),
    Among("tt", -1, -1, None),
];

static A_3: &'static [Among<Context>; 34] = &[
    Among("uad", -1, 18, None),
    Among("vad", -1, 19, None),
    Among("cid", -1, 20, None),
    Among("lid", -1, 21, None),
    Among("erid", -1, 22, None),
    Among("pand", -1, 23, None),
    Among("end", -1, 24, None),
    Among("ond", -1, 25, None),
    Among("lud", -1, 26, None),
    Among("rud", -1, 27, None),
    Among("ul", -1, 9, None),
    Among("her", -1, 28, None),
    Among("metr", -1, 7, None),
    Among("istr", -1, 6, None),
    Among("urs", -1, 5, None),
    Among("uct", -1, 2, None),
    Among("et", -1, 32, None),
    Among("mit", -1, 29, None),
    Among("ent", -1, 30, None),
    Among("umpt", -1, 3, None),
    Among("rpt", -1, 4, None),
    Among("ert", -1, 31, None),
    Among("yt", -1, 33, None),
    Among("iev", -1, 1, None),
    Among("olv", -1, 8, None),
    Among("ax", -1, 14, None),
    Among("ex", -1, 15, None),
    Among("bex", 26, 10, None),
    Among("dex", 26, 11, None),
    Among("pex", 26, 12, None),
    Among("tex", 26, 13, None),
    Among("ix", -1, 16, None),
    Among("lux", -1, 17, None),
    Among("yz", -1, 33, None),
];

#[derive(Clone)]
struct Context {}

fn r_A(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !env.hop_back(2) {
        return false;
    }
    return true;
}

fn r_B(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !env.hop_back(3) {
        return false;
    }
    return true;
}

fn r_C(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !env.hop_back(4) {
        return false;
    }
    return true;
}

fn r_D(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !env.hop_back(5) {
        return false;
    }
    return true;
}

fn r_E(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.hop_back(2) {
        return false;
    }
    env.cursor = env.limit - v_1;
    let v_2 = env.limit - env.cursor;
    'lab0: loop {
        if !env.eq_s_b(&"e") {
            break 'lab0;
        }
        return false;
    }
    env.cursor = env.limit - v_2;
    return true;
}

fn r_F(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.hop_back(3) {
        return false;
    }
    env.cursor = env.limit - v_1;
    let v_2 = env.limit - env.cursor;
    'lab0: loop {
        if !env.eq_s_b(&"e") {
            break 'lab0;
        }
        return false;
    }
    env.cursor = env.limit - v_2;
    return true;
}

fn r_G(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.hop_back(3) {
        return false;
    }
    env.cursor = env.limit - v_1;
    if !env.eq_s_b(&"f") {
        return false;
    }
    return true;
}

fn r_H(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.hop_back(2) {
        return false;
    }
    env.cursor = env.limit - v_1;
    'lab0: loop {
        let v_2 = env.limit - env.cursor;
        'lab1: loop {
            if !env.eq_s_b(&"t") {
                break 'lab1;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_2;
        if !env.eq_s_b(&"ll") {
            return false;
        }
        break 'lab0;
    }
    return true;
}

fn r_I(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.hop_back(2) {
        return false;
    }
    env.cursor = env.limit - v_1;
    let v_2 = env.limit - env.cursor;
    'lab0: loop {
        if !env.eq_s_b(&"o") {
            break 'lab0;
        }
        return false;
    }
    env.cursor = env.limit - v_2;
    let v_3 = env.limit - env.cursor;
    'lab1: loop {
        if !env.eq_s_b(&"e") {
            break 'lab1;
        }
        return false;
    }
    env.cursor = env.limit - v_3;
    return true;
}

fn r_J(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.hop_back(2) {
        return false;
    }
    env.cursor = env.limit - v_1;
    let v_2 = env.limit - env.cursor;
    'lab0: loop {
        if !env.eq_s_b(&"a") {
            break 'lab0;
        }
        return false;
    }
    env.cursor = env.limit - v_2;
    let v_3 = env.limit - env.cursor;
    'lab1: loop {
        if !env.eq_s_b(&"e") {
            break 'lab1;
        }
        return false;
    }
    env.cursor = env.limit - v_3;
    return true;
}

fn r_K(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.hop_back(3) {
        return false;
    }
    env.cursor = env.limit - v_1;
    'lab0: loop {
        let v_2 = env.limit - env.cursor;
        'lab1: loop {
            if !env.eq_s_b(&"l") {
                break 'lab1;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_2;
        'lab2: loop {
            if !env.eq_s_b(&"i") {
                break 'lab2;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_2;
        if !env.eq_s_b(&"e") {
            return false;
        }
        if env.cursor <= env.limit_backward {
            return false;
        }
        env.previous_char();
        if !env.eq_s_b(&"u") {
            return false;
        }
        break 'lab0;
    }
    return true;
}

fn r_L(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.hop_back(2) {
        return false;
    }
    env.cursor = env.limit - v_1;
    let v_2 = env.limit - env.cursor;
    'lab0: loop {
        if !env.eq_s_b(&"u") {
            break 'lab0;
        }
        return false;
    }
    env.cursor = env.limit - v_2;
    let v_3 = env.limit - env.cursor;
    'lab1: loop {
        if !env.eq_s_b(&"x") {
            break 'lab1;
        }
        return false;
    }
    env.cursor = env.limit - v_3;
    let v_4 = env.limit - env.cursor;
    'lab2: loop {
        if !env.eq_s_b(&"s") {
            break 'lab2;
        }
        let v_5 = env.limit - env.cursor;
        'lab3: loop {
            if !env.eq_s_b(&"o") {
                break 'lab3;
            }
            break 'lab2;
        }
        env.cursor = env.limit - v_5;
        return false;
    }
    env.cursor = env.limit - v_4;
    return true;
}

fn r_M(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.hop_back(2) {
        return false;
    }
    env.cursor = env.limit - v_1;
    let v_2 = env.limit - env.cursor;
    'lab0: loop {
        if !env.eq_s_b(&"a") {
            break 'lab0;
        }
        return false;
    }
    env.cursor = env.limit - v_2;
    let v_3 = env.limit - env.cursor;
    'lab1: loop {
        if !env.eq_s_b(&"c") {
            break 'lab1;
        }
        return false;
    }
    env.cursor = env.limit - v_3;
    let v_4 = env.limit - env.cursor;
    'lab2: loop {
        if !env.eq_s_b(&"e") {
            break 'lab2;
        }
        return false;
    }
    env.cursor = env.limit - v_4;
    let v_5 = env.limit - env.cursor;
    'lab3: loop {
        if !env.eq_s_b(&"m") {
            break 'lab3;
        }
        return false;
    }
    env.cursor = env.limit - v_5;
    return true;
}

fn r_N(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.hop_back(3) {
        return false;
    }
    env.cursor = env.limit - v_1;
    if !env.hop_back(2) {
        return false;
    }
    'lab0: loop {
        let v_2 = env.limit - env.cursor;
        'lab1: loop {
            let v_3 = env.limit - env.cursor;
            'lab2: loop {
                if !env.eq_s_b(&"s") {
                    break 'lab2;
                }
                break 'lab1;
            }
            env.cursor = env.limit - v_3;
            break 'lab0;
        }
        env.cursor = env.limit - v_2;
        if !env.hop_back(2) {
            return false;
        }
        break 'lab0;
    }
    return true;
}

fn r_O(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.hop_back(2) {
        return false;
    }
    env.cursor = env.limit - v_1;
    'lab0: loop {
        let v_2 = env.limit - env.cursor;
        'lab1: loop {
            if !env.eq_s_b(&"l") {
                break 'lab1;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_2;
        if !env.eq_s_b(&"i") {
            return false;
        }
        break 'lab0;
    }
    return true;
}

fn r_P(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.hop_back(2) {
        return false;
    }
    env.cursor = env.limit - v_1;
    let v_2 = env.limit - env.cursor;
    'lab0: loop {
        if !env.eq_s_b(&"c") {
            break 'lab0;
        }
        return false;
    }
    env.cursor = env.limit - v_2;
    return true;
}

fn r_Q(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.hop_back(2) {
        return false;
    }
    env.cursor = env.limit - v_1;
    let v_2 = env.limit - env.cursor;
    if !env.hop_back(3) {
        return false;
    }
    env.cursor = env.limit - v_2;
    let v_3 = env.limit - env.cursor;
    'lab0: loop {
        if !env.eq_s_b(&"l") {
            break 'lab0;
        }
        return false;
    }
    env.cursor = env.limit - v_3;
    let v_4 = env.limit - env.cursor;
    'lab1: loop {
        if !env.eq_s_b(&"n") {
            break 'lab1;
        }
        return false;
    }
    env.cursor = env.limit - v_4;
    return true;
}

fn r_R(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.hop_back(2) {
        return false;
    }
    env.cursor = env.limit - v_1;
    'lab0: loop {
        let v_2 = env.limit - env.cursor;
        'lab1: loop {
            if !env.eq_s_b(&"n") {
                break 'lab1;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_2;
        if !env.eq_s_b(&"r") {
            return false;
        }
        break 'lab0;
    }
    return true;
}

fn r_S(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.hop_back(2) {
        return false;
    }
    env.cursor = env.limit - v_1;
    'lab0: loop {
        let v_2 = env.limit - env.cursor;
        'lab1: loop {
            if !env.eq_s_b(&"dr") {
                break 'lab1;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_2;
        if !env.eq_s_b(&"t") {
            return false;
        }
        let v_3 = env.limit - env.cursor;
        'lab2: loop {
            if !env.eq_s_b(&"t") {
                break 'lab2;
            }
            return false;
        }
        env.cursor = env.limit - v_3;
        break 'lab0;
    }
    return true;
}

fn r_T(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.hop_back(2) {
        return false;
    }
    env.cursor = env.limit - v_1;
    'lab0: loop {
        let v_2 = env.limit - env.cursor;
        'lab1: loop {
            if !env.eq_s_b(&"s") {
                break 'lab1;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_2;
        if !env.eq_s_b(&"t") {
            return false;
        }
        let v_3 = env.limit - env.cursor;
        'lab2: loop {
            if !env.eq_s_b(&"o") {
                break 'lab2;
            }
            return false;
        }
        env.cursor = env.limit - v_3;
        break 'lab0;
    }
    return true;
}

fn r_U(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.hop_back(2) {
        return false;
    }
    env.cursor = env.limit - v_1;
    'lab0: loop {
        let v_2 = env.limit - env.cursor;
        'lab1: loop {
            if !env.eq_s_b(&"l") {
                break 'lab1;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_2;
        'lab2: loop {
            if !env.eq_s_b(&"m") {
                break 'lab2;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_2;
        'lab3: loop {
            if !env.eq_s_b(&"n") {
                break 'lab3;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_2;
        if !env.eq_s_b(&"r") {
            return false;
        }
        break 'lab0;
    }
    return true;
}

fn r_V(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.hop_back(2) {
        return false;
    }
    env.cursor = env.limit - v_1;
    if !env.eq_s_b(&"c") {
        return false;
    }
    return true;
}

fn r_W(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.hop_back(2) {
        return false;
    }
    env.cursor = env.limit - v_1;
    let v_2 = env.limit - env.cursor;
    'lab0: loop {
        if !env.eq_s_b(&"s") {
            break 'lab0;
        }
        return false;
    }
    env.cursor = env.limit - v_2;
    let v_3 = env.limit - env.cursor;
    'lab1: loop {
        if !env.eq_s_b(&"u") {
            break 'lab1;
        }
        return false;
    }
    env.cursor = env.limit - v_3;
    return true;
}

fn r_X(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.hop_back(2) {
        return false;
    }
    env.cursor = env.limit - v_1;
    'lab0: loop {
        let v_2 = env.limit - env.cursor;
        'lab1: loop {
            if !env.eq_s_b(&"l") {
                break 'lab1;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_2;
        'lab2: loop {
            if !env.eq_s_b(&"i") {
                break 'lab2;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_2;
        if !env.eq_s_b(&"e") {
            return false;
        }
        if env.cursor <= env.limit_backward {
            return false;
        }
        env.previous_char();
        if !env.eq_s_b(&"u") {
            return false;
        }
        break 'lab0;
    }
    return true;
}

fn r_Y(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.hop_back(2) {
        return false;
    }
    env.cursor = env.limit - v_1;
    if !env.eq_s_b(&"in") {
        return false;
    }
    return true;
}

fn r_Z(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.hop_back(2) {
        return false;
    }
    env.cursor = env.limit - v_1;
    let v_2 = env.limit - env.cursor;
    'lab0: loop {
        if !env.eq_s_b(&"f") {
            break 'lab0;
        }
        return false;
    }
    env.cursor = env.limit - v_2;
    return true;
}

fn r_AA(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.hop_back(2) {
        return false;
    }
    env.cursor = env.limit - v_1;
    if env.find_among_b(A_0, context) == 0 {
        return false;
    }
    return true;
}

fn r_BB(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.hop_back(3) {
        return false;
    }
    env.cursor = env.limit - v_1;
    let v_2 = env.limit - env.cursor;
    'lab0: loop {
        if !env.eq_s_b(&"met") {
            break 'lab0;
        }
        return false;
    }
    env.cursor = env.limit - v_2;
    let v_3 = env.limit - env.cursor;
    'lab1: loop {
        if !env.eq_s_b(&"ryst") {
            break 'lab1;
        }
        return false;
    }
    env.cursor = env.limit - v_3;
    return true;
}

fn r_CC(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.hop_back(2) {
        return false;
    }
    env.cursor = env.limit - v_1;
    if !env.eq_s_b(&"l") {
        return false;
    }
    return true;
}

fn r_endings(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if env.find_among_b(A_1, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !env.slice_del() {
        return false;
    }
    return true;
}

fn r_undouble(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if env.find_among_b(A_2, context) == 0 {
        return false;
    }
    env.cursor = env.limit - v_1;
    env.ket = env.cursor;
    if env.cursor <= env.limit_backward {
        return false;
    }
    env.previous_char();
    env.bra = env.cursor;
    if !env.slice_del() {
        return false;
    }
    return true;
}

fn r_respell(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_3, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if !env.slice_from("ief") {
                return false;
            }
        }
        2 => {
            if !env.slice_from("uc") {
                return false;
            }
        }
        3 => {
            if !env.slice_from("um") {
                return false;
            }
        }
        4 => {
            if !env.slice_from("rb") {
                return false;
            }
        }
        5 => {
            if !env.slice_from("ur") {
                return false;
            }
        }
        6 => {
            if !env.slice_from("ister") {
                return false;
            }
        }
        7 => {
            if !env.slice_from("meter") {
                return false;
            }
        }
        8 => {
            if !env.slice_from("olut") {
                return false;
            }
        }
        9 => {
            let v_1 = env.limit - env.cursor;
            'lab0: loop {
                if !env.eq_s_b(&"a") {
                    break 'lab0;
                }
                return false;
            }
            env.cursor = env.limit - v_1;
            let v_2 = env.limit - env.cursor;
            'lab1: loop {
                if !env.eq_s_b(&"i") {
                    break 'lab1;
                }
                return false;
            }
            env.cursor = env.limit - v_2;
            let v_3 = env.limit - env.cursor;
            'lab2: loop {
                if !env.eq_s_b(&"o") {
                    break 'lab2;
                }
                return false;
            }
            env.cursor = env.limit - v_3;
            if !env.slice_from("l") {
                return false;
            }
        }
        10 => {
            if !env.slice_from("bic") {
                return false;
            }
        }
        11 => {
            if !env.slice_from("dic") {
                return false;
            }
        }
        12 => {
            if !env.slice_from("pic") {
                return false;
            }
        }
        13 => {
            if !env.slice_from("tic") {
                return false;
            }
        }
        14 => {
            if !env.slice_from("ac") {
                return false;
            }
        }
        15 => {
            if !env.slice_from("ec") {
                return false;
            }
        }
        16 => {
            if !env.slice_from("ic") {
                return false;
            }
        }
        17 => {
            if !env.slice_from("luc") {
                return false;
            }
        }
        18 => {
            if !env.slice_from("uas") {
                return false;
            }
        }
        19 => {
            if !env.slice_from("vas") {
                return false;
            }
        }
        20 => {
            if !env.slice_from("cis") {
                return false;
            }
        }
        21 => {
            if !env.slice_from("lis") {
                return false;
            }
        }
        22 => {
            if !env.slice_from("eris") {
                return false;
            }
        }
        23 => {
            if !env.slice_from("pans") {
                return false;
            }
        }
        24 => {
            let v_4 = env.limit - env.cursor;
            'lab3: loop {
                if !env.eq_s_b(&"s") {
                    break 'lab3;
                }
                return false;
            }
            env.cursor = env.limit - v_4;
            if !env.slice_from("ens") {
                return false;
            }
        }
        25 => {
            if !env.slice_from("ons") {
                return false;
            }
        }
        26 => {
            if !env.slice_from("lus") {
                return false;
            }
        }
        27 => {
            if !env.slice_from("rus") {
                return false;
            }
        }
        28 => {
            let v_5 = env.limit - env.cursor;
            'lab4: loop {
                if !env.eq_s_b(&"p") {
                    break 'lab4;
                }
                return false;
            }
            env.cursor = env.limit - v_5;
            let v_6 = env.limit - env.cursor;
            'lab5: loop {
                if !env.eq_s_b(&"t") {
                    break 'lab5;
                }
                return false;
            }
            env.cursor = env.limit - v_6;
            if !env.slice_from("hes") {
                return false;
            }
        }
        29 => {
            if !env.slice_from("mis") {
                return false;
            }
        }
        30 => {
            let v_7 = env.limit - env.cursor;
            'lab6: loop {
                if !env.eq_s_b(&"m") {
                    break 'lab6;
                }
                return false;
            }
            env.cursor = env.limit - v_7;
            if !env.slice_from("ens") {
                return false;
            }
        }
        31 => {
            if !env.slice_from("ers") {
                return false;
            }
        }
        32 => {
            let v_8 = env.limit - env.cursor;
            'lab7: loop {
                if !env.eq_s_b(&"n") {
                    break 'lab7;
                }
                return false;
            }
            env.cursor = env.limit - v_8;
            if !env.slice_from("es") {
                return false;
            }
        }
        33 => {
            if !env.slice_from("ys") {
                return false;
            }
        }
        _ => (),
    }
    return true;
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {};
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    let v_1 = env.limit - env.cursor;
    r_endings(env, context);
    env.cursor = env.limit - v_1;
    let v_2 = env.limit - env.cursor;
    r_undouble(env, context);
    env.cursor = env.limit - v_2;
    let v_3 = env.limit - env.cursor;
    r_respell(env, context);
    env.cursor = env.limit - v_3;
    env.cursor = env.limit_backward;
    return true;
}
