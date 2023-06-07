use std::ops::RangeInclusive;

use brl::BraillePattern;
use brl::BraillePattern::*;

// Universal signs:
pub const NUMBER_SIGN: BraillePattern = Braille3456;
pub const PLACEHOLDER: BraillePattern = Braille123456;

// Script and font prefixes:
pub const SMALL_GREEK_PREFIX: BraillePattern = Braille56;
pub const CAPITAL_GREEK_PREFIX: BraillePattern = Braille456;
pub const SMALL_LATIN_PREFIX: BraillePattern = Braille6;
pub const CAPITAL_LATIN_PREFIX: BraillePattern = Braille46;
pub const SMALL_FRAKTUR_PREFIX: BraillePattern = Braille5;
pub const CAPITAL_FRAKTUR_PREFIX: BraillePattern = Braille45;
pub const BOLD_PREFIX: BraillePattern = Braille1456;
pub const CALLIGRAPHIC_PREFIX: BraillePattern = Braille346;
pub const BOLD_CALLIGRAPHIC_PREFIX: BraillePattern = Braille3456;
pub const DOUBLESTRUCK_PREFIX: BraillePattern = Braille12456;

// Alphabets:
pub const LATIN_ALPHABET: [BraillePattern; 26] = [
    Braille1,
    Braille12,
    Braille14,
    Braille145,
    Braille15,
    Braille124,
    Braille1245,
    Braille125,
    Braille24,
    Braille245,
    Braille13,
    Braille123,
    Braille134,
    Braille1345,
    Braille135,
    Braille1234,
    Braille12345,
    Braille1235,
    Braille234,
    Braille2345,
    Braille136,
    Braille1236,
    Braille2456,
    Braille1346,
    Braille13456,
    Braille1356,
];

// Note: sigma is repeated.
pub const GREEK_ALPHABET: [BraillePattern; 25] = [
    Braille1,
    Braille12,
    Braille1245,
    Braille145,
    Braille15,
    Braille1356,
    Braille245,
    Braille125,
    Braille24,
    Braille13,
    Braille123,
    Braille134,
    Braille1345,
    Braille1346,
    Braille135,
    Braille1234,
    Braille1235,
    Braille234,
    Braille234,
    Braille2345,
    Braille136,
    Braille124,
    Braille14,
    Braille13456,
    Braille2456,
];

// Unicode ranges for various combinations of font and script from the Mathematical Alphanumeric Symbols block:
pub const CAPITAL_LATIN_RANGE: RangeInclusive<u32> = 0x41..=0x5a;
pub const CAPITAL_LATIN_BOLD_RANGE: RangeInclusive<u32> = 0x1d400..=0x1d419;
pub const CAPITAL_LATIN_CALLIGRAPHIC_RANGE: RangeInclusive<u32> = 0x1d49c..=0x1d4b5;
pub const CAPITAL_LATIN_BOLD_CALLIGRAPHIC_RANGE: RangeInclusive<u32> = 0x1d4d0..=0x1d4e9;
pub const CAPITAL_FRAKTUR_RANGE: RangeInclusive<u32> = 0x1d504..=0x1d51d;
pub const CAPITAL_LATIN_DOUBLESTRUCK_RANGE: RangeInclusive<u32> = 0x1d538..=0x1d551;

pub const SMALL_LATIN_RANGE: RangeInclusive<u32> = 0x61..=0x7a;
pub const SMALL_LATIN_BOLD_RANGE: RangeInclusive<u32> = 0x1d41a..=0x1d435;
pub const SMALL_LATIN_CALLIGRAPHIC_RANGE: RangeInclusive<u32> = 0x1d4b6..=0x1d4cf;
pub const SMALL_LATIN_BOLD_CALLIGRAPHIC_RANGE: RangeInclusive<u32> = 0x1d4ea..=0x1d503;
pub const SMALL_FRAKTUR_RANGE: RangeInclusive<u32> = 0x1d51e..=0x1d537;
pub const SMALL_LATIN_DOUBLESTRUCK_RANGE: RangeInclusive<u32> = 0x1d552..=0x1d56b;

pub const CAPITAL_GREEK_RANGE: RangeInclusive<u32> = 0x391..=0x3a9;
pub const CAPITAL_GREEK_BOLD_RANGE: RangeInclusive<u32> = 0x1d6a8..=0x1d6c0;
pub const SMALL_GREEK_RANGE: RangeInclusive<u32> = 0x3b1..=0x3c9;
pub const SMALL_GREEK_BOLD_RANGE: RangeInclusive<u32> = 0x1d6c2..=0x1d6da;
