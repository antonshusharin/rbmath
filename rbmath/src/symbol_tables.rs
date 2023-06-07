use brl::{braille, BrailleString};

use crate::math_element::SubscriptType;

pub fn normalize(c: char) -> Option<char> {
    if (0x2061..=0x2064).contains(&(c as u32)) {
        None
    } else {
        Some(match c {
            'ϵ' => 'ε',
            '𝛜' => '𝛆',
            'ϴ' => 'Θ',
            '𝚹' => '𝚯',
            'ϑ' => 'θ',
            '𝛝' => '𝛉',
            '−' => '-',
            '⩾' => '≥',
            '⩽' => '≤',

            'ℎ' => '𝑕',
            'ℬ' => '𝒝',
            'ℰ' => '𝒠',
            'ℱ' => '𝒡',
            'ℋ' => '𝒣',
            'ℐ' => '𝒤',
            'ℒ' => '𝒧',
            'ℳ' => '𝒨',
            'ℛ' => '𝒭',
            'ℯ' => '𝒺',
            'ℊ' => '𝒼',
            'ℴ' => '𝓄',
            'ℭ' => '𝔆',
            'ℌ' => '𝔋',
            'ℑ' => '𝔌',
            'ℜ' => '𝔕',
            'ℨ' => '𝔝',
            'ℂ' => '𝔺',
            'ℍ' => '𝔿',
            'ℕ' => '𝕅',
            'ℙ' => '𝕇',
            'ℚ' => '𝕈',
            'ℝ' => '𝕉',
            'ℤ' => '𝕑',
            other => other,
        })
    }
}

pub fn special_function(f: &str) -> Option<BrailleString> {
    match f {
        "sin" => Some(braille!(1246, 234)),
        "cos" => Some(braille!(1246, 13)),
        "tan" => Some(braille!(1246, 2345)),
        "cot" | "ctg" => Some(braille!(1246,13,2345)),
        "log" => Some(braille!(1246, 123)),
        "ln" => Some(braille!(1246,123,1345)),
        "lg" => Some(braille!(1246,123,1245)),
        "lim" => Some(braille!(1246, 123, 134)),
        _ => None,
    }
}

pub (crate) fn has_affinity(c: char, script: SubscriptType) -> bool {
    match c {
        '′' | '+' | '-' | '∗' if script == SubscriptType::Sup => true,
        '~' | '˙' | '^' if script == SubscriptType::Over => true,
        _ => false
    }
}


pub fn braille_symbol(c: char) -> BrailleString {
    match c {
        '+' => braille!(235),
        '-' => braille!(36),
        '±' => braille!(235, 36),
        '∓' => braille!(36, 235),
        '×' => braille!(236),
        '⋅' => braille!(3),

        '=' => braille!(0, 2356),
        '>' => braille!(0, 135, 0),
        '<' => braille!(0, 246, 0),
        '≥' => braille!(0, 135, 2356),
        '≤' => braille!(0, 246, 2356),

        ',' => braille!(6, 2),
        '…' => braille!(6, 3),
        ':' => braille!(6, 25,0),
        '!' => braille!(6,235),

        '∑' => braille!(456, 234),
        '∏' => braille!(456, 1234),

        '∫' => braille!(2346),
        '∬' => braille!(2346, 2346),
        '∭' => braille!(2346, 2346, 2346),
        '∂' => braille!(1456),
        '′' => braille!(35),

'∈' => braille!(0,5,246,0),
'∉' => braille!(0,45,246,0),
'⊂' => braille!(0,12346,0),
'∅' => braille!(4,356),
'∪' => braille!(0,56,356),
'∩' => braille!(0,56,256),
'∖' => braille!(56,256),

'∧' => braille!(0,56,236),
'∨' => braille!(0,56,35),
'¬' => braille!(26),
'∀' => braille!(1246,3),
'∃' => braille!(1246,26),


        '→' => braille!(0, 25, 135),
        '←' => braille!(0,246,25),

'‾' => braille!(25),
'^' => braille!(256),
'~' => braille!(26),
'∗' => braille!(23),
'˙' => braille!(2),

        '(' => braille!(126),
        ')' => braille!(345),
        '|' => braille!(456),
        '[' => braille!(12356),
        ']' => braille!(23456),

        other => {
            log::warn!("Unrecognized symbol: {}", other);
            braille!(123456)
        }
    }
}
