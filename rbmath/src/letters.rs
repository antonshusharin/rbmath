use std::ops::RangeInclusive;

use brl::{BraillePattern, BrailleString};

use crate::constants::{
    BOLD_CALLIGRAPHIC_PREFIX, BOLD_PREFIX, CALLIGRAPHIC_PREFIX, CAPITAL_FRAKTUR_PREFIX,
    CAPITAL_GREEK_PREFIX, CAPITAL_LATIN_PREFIX, DOUBLESTRUCK_PREFIX, GREEK_ALPHABET,
    LATIN_ALPHABET, SMALL_FRAKTUR_PREFIX, SMALL_GREEK_PREFIX, SMALL_LATIN_PREFIX,
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Script {
    Latin,
    Greek,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Font {
    Normal,
    Bold,
    Fraktur,
    Calligraphic,
    BoldCalligraphic,
    Doublestruck,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Classifier {
    pub script: Script,
    pub font: Font,
    pub capital: bool,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Letter(pub usize, pub Classifier);

fn try_range(
    c: char,
    range: &RangeInclusive<u32>,
    script: Script,
    font: Font,
    capital: bool,
) -> Option<(Classifier, &RangeInclusive<u32>)> {
    if range.contains(&(c as u32)) {
        Some((
            Classifier {
                script,
                font,
                capital,
            },
            range,
        ))
    } else {
        None
    }
}

impl Classifier {
    pub fn get_prefix(&self) -> BrailleString {
        if self.font == Font::Fraktur {
            return if self.capital {
                BrailleString::from(CAPITAL_FRAKTUR_PREFIX)
            } else {
                BrailleString::from(SMALL_FRAKTUR_PREFIX)
            };
        }
        let mut res = BrailleString::new();
        res += match self.script {
            Script::Latin if self.capital => CAPITAL_LATIN_PREFIX,
            Script::Latin => SMALL_LATIN_PREFIX,
            Script::Greek if self.capital => CAPITAL_GREEK_PREFIX,
            Script::Greek => SMALL_GREEK_PREFIX,
        };
        let font_prefix = match self.font {
            Font::Normal | Font::Fraktur => None,
            Font::Bold => Some(BOLD_PREFIX),
            Font::Calligraphic => Some(CALLIGRAPHIC_PREFIX),
            Font::BoldCalligraphic => Some(BOLD_CALLIGRAPHIC_PREFIX),
            Font::Doublestruck => Some(DOUBLESTRUCK_PREFIX),
        };
        if let Some(font_prefix) = font_prefix {
            res += font_prefix;
        }
        res
    }

    pub fn classify(c: char) -> Option<Letter> {
        use crate::constants::*;
        use Font::*;
        use Script::*;
        let cls = if let Some(r) = try_range(c, &SMALL_LATIN_RANGE, Latin, Normal, false) {
            r
        } else if let Some(r) = try_range(c, &SMALL_LATIN_BOLD_RANGE, Latin, Bold, false) {
            r
        } else if let Some(r) = try_range(
            c,
            &SMALL_LATIN_CALLIGRAPHIC_RANGE,
            Latin,
            Calligraphic,
            false,
        ) {
            r
        } else if let Some(r) = try_range(
            c,
            &SMALL_LATIN_BOLD_CALLIGRAPHIC_RANGE,
            Latin,
            BoldCalligraphic,
            false,
        ) {
            r
        } else if let Some(r) = try_range(
            c,
            &SMALL_LATIN_DOUBLESTRUCK_RANGE,
            Latin,
            Doublestruck,
            false,
        ) {
            r
        } else if let Some(r) = try_range(c, &SMALL_FRAKTUR_RANGE, Latin, Fraktur, false) {
            r
        } else if let Some(r) = try_range(c, &CAPITAL_LATIN_RANGE, Latin, Normal, true) {
            r
        } else if let Some(r) = try_range(c, &CAPITAL_LATIN_BOLD_RANGE, Latin, Bold, true) {
            r
        } else if let Some(r) = try_range(
            c,
            &CAPITAL_LATIN_CALLIGRAPHIC_RANGE,
            Latin,
            Calligraphic,
            true,
        ) {
            r
        } else if let Some(r) = try_range(
            c,
            &CAPITAL_LATIN_BOLD_CALLIGRAPHIC_RANGE,
            Latin,
            BoldCalligraphic,
            true,
        ) {
            r
        } else if let Some(r) = try_range(
            c,
            &CAPITAL_LATIN_DOUBLESTRUCK_RANGE,
            Latin,
            Doublestruck,
            true,
        ) {
            r
        } else if let Some(r) = try_range(c, &CAPITAL_FRAKTUR_RANGE, Latin, Fraktur, true) {
            r
        } else if let Some(r) = try_range(c, &SMALL_GREEK_RANGE, Greek, Normal, false) {
            r
        } else if let Some(r) = try_range(c, &SMALL_GREEK_BOLD_RANGE, Greek, Bold, false) {
            r
        } else if let Some(r) = try_range(c, &CAPITAL_GREEK_RANGE, Greek, Normal, true) {
            r
        } else if let Some(r) = try_range(c, &CAPITAL_GREEK_BOLD_RANGE, Greek, Bold, true) {
            r
        } else {
            return None;
        };
        Some(Letter((c as u32 - cls.1.start()) as usize, cls.0))
    }
}

impl Letter {
    pub fn get_rendering(&self) -> BraillePattern {
        match self.1.script {
            Script::Latin => LATIN_ALPHABET[self.0],
            Script::Greek => GREEK_ALPHABET[self.0],
        }
    }
}
