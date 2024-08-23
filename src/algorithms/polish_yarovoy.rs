/*
 * Polish language stemmer
 *
 * This algorithm has been ported to Rust from Nikolay Yarovoy's
 * Go port. Here is the Go version: https://github.com/nickspring/simple_polish_stemmer
 *
 * Nikolay Yarovoy's Go version is in turn inspired by Błażej Kubiński's
 * Python implementation. Here is the pra-original: https://github.com/Tutanchamon/pl_stemmer
 *
 * Both the original Python and Go versions of the algorithm
 * have been published under the MIT license.
 */

use alloc::borrow::Cow;
use alloc::string::String;
use daachorse::DoubleArrayAhoCorasick;
use precis_core::profile::PrecisFastInvocation;
use precis_profiles::UsernameCaseMapped;
use unicode_normalization::UnicodeNormalization;

struct Rule {
    min_word_len: usize,
    left_shift: usize,
    right_shift: usize,
    suffixes_accented: &'static [&'static str],
    suffixes_unaccented: &'static [&'static str],
}

const RULES: &'static [Rule] = &[
    // Remove nouns
    Rule {
        min_word_len: 8,
        left_shift: 0,
        right_shift: 4,
        suffixes_accented: &["zacja", "zacją", "zacji"],
        suffixes_unaccented: &[],
    },
    Rule {
        min_word_len: 7,
        left_shift: 0,
        right_shift: 4,
        suffixes_accented: &[
            "acja", "acji", "acją", "tach", "anie", "enie", "eniu", "aniu",
        ],
        suffixes_unaccented: &[],
    },
    Rule {
        min_word_len: 7,
        left_shift: 0,
        right_shift: 2,
        suffixes_accented: &["tyka"],
        suffixes_unaccented: &[],
    },
    Rule {
        min_word_len: 6,
        left_shift: 0,
        right_shift: 3,
        suffixes_accented: &["ach", "ami", "nia", "niu", "cia", "ciu"],
        suffixes_unaccented: &[],
    },
    Rule {
        min_word_len: 6,
        left_shift: 0,
        right_shift: 2,
        suffixes_accented: &["cji", "cja", "cją"],
        suffixes_unaccented: &[],
    },
    Rule {
        min_word_len: 6,
        left_shift: 0,
        right_shift: 2,
        suffixes_accented: &["ce", "ta"],
        suffixes_unaccented: &[],
    },
    // Diminutive
    Rule {
        min_word_len: 7,
        left_shift: 0,
        right_shift: 5,
        suffixes_accented: &["eczek", "iczek", "iszek", "aszek", "uszek"],
        suffixes_unaccented: &[],
    },
    Rule {
        min_word_len: 7,
        left_shift: 0,
        right_shift: 2,
        suffixes_accented: &["enek", "ejek", "erek"],
        suffixes_unaccented: &[],
    },
    Rule {
        min_word_len: 5,
        left_shift: 0,
        right_shift: 2,
        suffixes_accented: &["ek", "ak"],
        suffixes_unaccented: &[],
    },
    // Remove adjectives ends
    Rule {
        min_word_len: 8,
        left_shift: 3,
        right_shift: 3,
        suffixes_accented: &["naj", "sze", "szy"],
        suffixes_unaccented: &[],
    },
    Rule {
        min_word_len: 8,
        left_shift: 3,
        right_shift: 5,
        suffixes_accented: &["naj", "szych"],
        suffixes_unaccented: &[],
    },
    Rule {
        min_word_len: 7,
        left_shift: 0,
        right_shift: 4,
        suffixes_accented: &["czny"],
        suffixes_unaccented: &[],
    },
    Rule {
        min_word_len: 6,
        left_shift: 0,
        right_shift: 3,
        suffixes_accented: &["owy", "owa", "owe", "ych", "ego"],
        suffixes_unaccented: &[],
    },
    Rule {
        min_word_len: 6,
        left_shift: 0,
        right_shift: 2,
        suffixes_accented: &["ej"],
        suffixes_unaccented: &[],
    },
    // Remove verbs ends
    Rule {
        min_word_len: 6,
        left_shift: 0,
        right_shift: 3,
        suffixes_accented: &["bym"],
        suffixes_unaccented: &[],
    },
    Rule {
        min_word_len: 6,
        left_shift: 0,
        right_shift: 3,
        suffixes_accented: &["esz", "asz", "cie", "eść", "aść", "łem", "amy", "emy"],
        suffixes_unaccented: &["esz", "asz", "cie", "esc", "asc", "lem", "amy", "emy"],
    },
    Rule {
        min_word_len: 4,
        left_shift: 0,
        right_shift: 2,
        suffixes_accented: &["esz", "asz", "eść", "aść", "eć", "ać"],
        suffixes_unaccented: &["esz", "asz", "esc", "asc", "ec", "ac"],
    },
    Rule {
        min_word_len: 4,
        left_shift: 0,
        right_shift: 1,
        suffixes_accented: &["aj"],
        suffixes_unaccented: &[],
    },
    Rule {
        min_word_len: 4,
        left_shift: 0,
        right_shift: 2,
        suffixes_accented: &["ać", "em", "am", "ał", "ił", "ić", "ąc"],
        suffixes_unaccented: &["ac", "em", "am", "al", "il", "ic", "ac"],
    },
    // Remove adverbs ends
    Rule {
        min_word_len: 5,
        left_shift: 0,
        right_shift: 2,
        suffixes_accented: &["nie", "wie"],
        suffixes_unaccented: &[],
    },
    Rule {
        min_word_len: 5,
        left_shift: 0,
        right_shift: 2,
        suffixes_accented: &["rze"],
        suffixes_unaccented: &[],
    },
    // Remove plural forms
    Rule {
        min_word_len: 5,
        left_shift: 0,
        right_shift: 2,
        suffixes_accented: &["ów", "om"],
        suffixes_unaccented: &["ow", "om"],
    },
    Rule {
        min_word_len: 5,
        left_shift: 0,
        right_shift: 3,
        suffixes_accented: &["ami"],
        suffixes_unaccented: &[],
    },
    // General ends
    Rule {
        min_word_len: 5,
        left_shift: 0,
        right_shift: 2,
        suffixes_accented: &["ia", "ie"],
        suffixes_unaccented: &[],
    },
    Rule {
        min_word_len: 5,
        left_shift: 0,
        right_shift: 1,
        suffixes_accented: &["u", "ą", "i", "a", "ę", "y", "ę", "ł"],
        suffixes_unaccented: &["u", "a", "i", "e", "y", "l"],
    },
];

// fn unaccent<'a>(input: &'a str) -> String {
//     let mut value: String = input.nfd().collect::<String>();
//
//     value = match UsernameCaseMapped::enforce(value.as_str()) {
//         Ok(value) => {
//             value.into()
//         },
//         Err(_) => {
//             "".into()
//         },
//     };
//
//     value = match AhoCorasick::new(&["ą", "ć", "ę", "ł", "ń", "ó", "ś", "ż", "ź"]) {
//         Ok(patterns) => {
//             patterns.replace_all(value.as_str(), &["a", "c", "e", "l", "n", "o", "s", "z", "z"]).into()
//         },
//         Err(_) => {
//             "".into()
//         },
//     };
//
//     value.nfc().collect::<String>()
// }

fn unaccent(input: &str) -> String {
    // Step 1: Normalize to NFD (Normalization Form Decomposition)
    let mut value: String = input.nfd().collect::<String>();

    // Step 2: Enforce UsernameCaseMapped (This seems to be related to IDNA)
    value = match UsernameCaseMapped::enforce(value.as_str()) {
        Ok(value) => value.into(),
        Err(_) => "".into(),
    };

    // Step 3: Replace accented characters with their unaccented equivalents using daachorse
    let patterns = ["ą", "ć", "ę", "ł", "ń", "ó", "ś", "ż", "ź"];
    let replacements = ["a", "c", "e", "l", "n", "o", "s", "z", "z"];

    // Specify the type parameter for DoubleArrayAhoCorasick
    let ac: DoubleArrayAhoCorasick<u32> = DoubleArrayAhoCorasick::new(&patterns).unwrap();
    let mut result = String::new();
    let mut last_index = 0;

    // Use find_iter to find and replace matches
    for mat in ac.find_iter(value.as_str()) {
        // mat.start() and mat.end() give the span of the matched pattern
        result.push_str(&value[last_index..mat.start()]);

        // We need to map the matched pattern to its replacement
        let matched_pattern = &value[mat.start()..mat.end()];
        if let Some(index) = patterns.iter().position(|&p| p == matched_pattern) {
            result.push_str(replacements[index]);
        } else {
            result.push_str(matched_pattern); // if no replacement, keep original
        }

        last_index = mat.end();
    }
    result.push_str(&value[last_index..]);

    // Step 4: Normalize back to NFC (Normalization Form Composition)
    result.nfc().collect::<String>()
}

pub fn stem<'a>(input: &'a str, enable_unaccent: bool) -> Cow<'a, str> {
    let mut output = input.into();

    if enable_unaccent {
        output = unaccent(input);
    }

    for rule in RULES {
        if output.chars().count() < rule.min_word_len {
            continue;
        }

        let mut suffixes = rule.suffixes_accented;
        if enable_unaccent && rule.suffixes_unaccented.len() > 0 {
            suffixes = rule.suffixes_unaccented;
        }

        let mut has_suffix = false;
        for suffix in suffixes {
            has_suffix = output.ends_with(suffix);

            if has_suffix {
                break;
            }
        }

        if has_suffix {
            let mut output_chars = output.chars();

            for _ in 0..rule.left_shift {
                output_chars.next();
            }

            for _ in 0..rule.right_shift {
                output_chars.next_back();
            }

            output = output_chars.as_str().into();
        }
    }

    Cow::from(output)
}
