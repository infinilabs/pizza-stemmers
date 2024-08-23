//! Generated by Snowball 2.2.0 - https://snowballstem.org/

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
use super::super::among::Among;
use super::super::env::SnowballEnv;

static A_0: &'static [Among<Context>; 37] = &[
    Among("a", -1, 1, None),
    Among("arna", 0, 1, None),
    Among("erna", 0, 1, None),
    Among("heterna", 2, 1, None),
    Among("orna", 0, 1, None),
    Among("ad", -1, 1, None),
    Among("e", -1, 1, None),
    Among("ade", 6, 1, None),
    Among("ande", 6, 1, None),
    Among("arne", 6, 1, None),
    Among("are", 6, 1, None),
    Among("aste", 6, 1, None),
    Among("en", -1, 1, None),
    Among("anden", 12, 1, None),
    Among("aren", 12, 1, None),
    Among("heten", 12, 1, None),
    Among("ern", -1, 1, None),
    Among("ar", -1, 1, None),
    Among("er", -1, 1, None),
    Among("heter", 18, 1, None),
    Among("or", -1, 1, None),
    Among("s", -1, 2, None),
    Among("as", 21, 1, None),
    Among("arnas", 22, 1, None),
    Among("ernas", 22, 1, None),
    Among("ornas", 22, 1, None),
    Among("es", 21, 1, None),
    Among("ades", 26, 1, None),
    Among("andes", 26, 1, None),
    Among("ens", 21, 1, None),
    Among("arens", 29, 1, None),
    Among("hetens", 29, 1, None),
    Among("erns", 21, 1, None),
    Among("at", -1, 1, None),
    Among("andet", -1, 1, None),
    Among("het", -1, 1, None),
    Among("ast", -1, 1, None),
];

static A_1: &'static [Among<Context>; 7] = &[
    Among("dd", -1, -1, None),
    Among("gd", -1, -1, None),
    Among("nn", -1, -1, None),
    Among("dt", -1, -1, None),
    Among("gt", -1, -1, None),
    Among("kt", -1, -1, None),
    Among("tt", -1, -1, None),
];

static A_2: &'static [Among<Context>; 5] = &[
    Among("ig", -1, 1, None),
    Among("lig", 0, 1, None),
    Among("els", -1, 1, None),
    Among("fullt", -1, 3, None),
    Among("\u{00F6}st", -1, 2, None),
];

static G_v: &'static [u8; 19] = &[17, 65, 16, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 32];

static G_s_ending: &'static [u8; 3] = &[119, 127, 149];

static G_ost_ending: &'static [u8; 2] = &[173, 58];

#[derive(Clone)]
struct Context {
    i_x: i32,
    i_p1: i32,
}

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    context.i_p1 = env.limit;
    let v_1 = env.cursor;
    if !env.hop(3) {
        return false;
    }
    context.i_x = env.cursor;
    env.cursor = v_1;
    'golab0: loop {
        let v_2 = env.cursor;
        'lab1: loop {
            if !env.in_grouping(G_v, 97, 246) {
                break 'lab1;
            }
            env.cursor = v_2;
            break 'golab0;
        }
        env.cursor = v_2;
        if env.cursor >= env.limit {
            return false;
        }
        env.next_char();
    }
    'golab2: loop {
        'lab3: loop {
            if !env.out_grouping(G_v, 97, 246) {
                break 'lab3;
            }
            break 'golab2;
        }
        if env.cursor >= env.limit {
            return false;
        }
        env.next_char();
    }
    context.i_p1 = env.cursor;
    'lab4: loop {
        if context.i_p1 >= context.i_x {
            break 'lab4;
        }
        context.i_p1 = context.i_x;
        break 'lab4;
    }
    return true;
}

fn r_main_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_2 = env.limit_backward;
    env.limit_backward = context.i_p1;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_0, context);
    if among_var == 0 {
        env.limit_backward = v_2;
        return false;
    }
    env.bra = env.cursor;
    env.limit_backward = v_2;
    match among_var {
        1 => {
            if !env.slice_del() {
                return false;
            }
        }
        2 => {
            if !env.in_grouping_b(G_s_ending, 98, 121) {
                return false;
            }
            if !env.slice_del() {
                return false;
            }
        }
        _ => (),
    }
    return true;
}

fn r_consonant_pair(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_2 = env.limit_backward;
    env.limit_backward = context.i_p1;
    let v_3 = env.limit - env.cursor;
    if env.find_among_b(A_1, context) == 0 {
        env.limit_backward = v_2;
        return false;
    }
    env.cursor = env.limit - v_3;
    env.ket = env.cursor;
    if env.cursor <= env.limit_backward {
        env.limit_backward = v_2;
        return false;
    }
    env.previous_char();
    env.bra = env.cursor;
    if !env.slice_del() {
        return false;
    }
    env.limit_backward = v_2;
    return true;
}

fn r_other_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_2 = env.limit_backward;
    env.limit_backward = context.i_p1;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_2, context);
    if among_var == 0 {
        env.limit_backward = v_2;
        return false;
    }
    env.bra = env.cursor;
    env.limit_backward = v_2;
    match among_var {
        1 => {
            if !env.slice_del() {
                return false;
            }
        }
        2 => {
            if !env.in_grouping_b(G_ost_ending, 105, 118) {
                return false;
            }
            if !env.slice_from("\u{00F6}s") {
                return false;
            }
        }
        3 => {
            if !env.slice_from("full") {
                return false;
            }
        }
        _ => (),
    }
    return true;
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context { i_x: 0, i_p1: 0 };
    let v_1 = env.cursor;
    r_mark_regions(env, context);
    env.cursor = v_1;
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    let v_2 = env.limit - env.cursor;
    r_main_suffix(env, context);
    env.cursor = env.limit - v_2;
    let v_3 = env.limit - env.cursor;
    r_consonant_pair(env, context);
    env.cursor = env.limit - v_3;
    let v_4 = env.limit - env.cursor;
    r_other_suffix(env, context);
    env.cursor = env.limit - v_4;
    env.cursor = env.limit_backward;
    return true;
}
