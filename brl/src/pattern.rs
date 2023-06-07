use std::ops::{Add, AddAssign};

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BraillePattern {
    BrailleEmpty = 0,
    Braille1 = 1,
    Braille2 = 2,
    Braille12 = 3,
    Braille3 = 4,
    Braille13 = 5,
    Braille23 = 6,
    Braille123 = 7,
    Braille4 = 8,
    Braille14 = 9,
    Braille24 = 10,
    Braille124 = 11,
    Braille34 = 12,
    Braille134 = 13,
    Braille234 = 14,
    Braille1234 = 15,
    Braille5 = 16,
    Braille15 = 17,
    Braille25 = 18,
    Braille125 = 19,
    Braille35 = 20,
    Braille135 = 21,
    Braille235 = 22,
    Braille1235 = 23,
    Braille45 = 24,
    Braille145 = 25,
    Braille245 = 26,
    Braille1245 = 27,
    Braille345 = 28,
    Braille1345 = 29,
    Braille2345 = 30,
    Braille12345 = 31,
    Braille6 = 32,
    Braille16 = 33,
    Braille26 = 34,
    Braille126 = 35,
    Braille36 = 36,
    Braille136 = 37,
    Braille236 = 38,
    Braille1236 = 39,
    Braille46 = 40,
    Braille146 = 41,
    Braille246 = 42,
    Braille1246 = 43,
    Braille346 = 44,
    Braille1346 = 45,
    Braille2346 = 46,
    Braille12346 = 47,
    Braille56 = 48,
    Braille156 = 49,
    Braille256 = 50,
    Braille1256 = 51,
    Braille356 = 52,
    Braille1356 = 53,
    Braille2356 = 54,
    Braille12356 = 55,
    Braille456 = 56,
    Braille1456 = 57,
    Braille2456 = 58,
    Braille12456 = 59,
    Braille3456 = 60,
    Braille13456 = 61,
    Braille23456 = 62,
    Braille123456 = 63,
    BrailleNewline,
}

impl BraillePattern {
    pub const fn from_dots(dots: usize) -> Self {
        match dots {
            0 => BraillePattern::BrailleEmpty,
            1 => BraillePattern::Braille1,
            2 => BraillePattern::Braille2,
            12 => BraillePattern::Braille12,
            3 => BraillePattern::Braille3,
            13 => BraillePattern::Braille13,
            23 => BraillePattern::Braille23,
            123 => BraillePattern::Braille123,
            4 => BraillePattern::Braille4,
            14 => BraillePattern::Braille14,
            24 => BraillePattern::Braille24,
            124 => BraillePattern::Braille124,
            34 => BraillePattern::Braille34,
            134 => BraillePattern::Braille134,
            234 => BraillePattern::Braille234,
            1234 => BraillePattern::Braille1234,
            5 => BraillePattern::Braille5,
            15 => BraillePattern::Braille15,
            25 => BraillePattern::Braille25,
            125 => BraillePattern::Braille125,
            35 => BraillePattern::Braille35,
            135 => BraillePattern::Braille135,
            235 => BraillePattern::Braille235,
            1235 => BraillePattern::Braille1235,
            45 => BraillePattern::Braille45,
            145 => BraillePattern::Braille145,
            245 => BraillePattern::Braille245,
            1245 => BraillePattern::Braille1245,
            345 => BraillePattern::Braille345,
            1345 => BraillePattern::Braille1345,
            2345 => BraillePattern::Braille2345,
            12345 => BraillePattern::Braille12345,
            6 => BraillePattern::Braille6,
            16 => BraillePattern::Braille16,
            26 => BraillePattern::Braille26,
            126 => BraillePattern::Braille126,
            36 => BraillePattern::Braille36,
            136 => BraillePattern::Braille136,
            236 => BraillePattern::Braille236,
            1236 => BraillePattern::Braille1236,
            46 => BraillePattern::Braille46,
            146 => BraillePattern::Braille146,
            246 => BraillePattern::Braille246,
            1246 => BraillePattern::Braille1246,
            346 => BraillePattern::Braille346,
            1346 => BraillePattern::Braille1346,
            2346 => BraillePattern::Braille2346,
            12346 => BraillePattern::Braille12346,
            56 => BraillePattern::Braille56,
            156 => BraillePattern::Braille156,
            256 => BraillePattern::Braille256,
            1256 => BraillePattern::Braille1256,
            356 => BraillePattern::Braille356,
            1356 => BraillePattern::Braille1356,
            2356 => BraillePattern::Braille2356,
            12356 => BraillePattern::Braille12356,
            456 => BraillePattern::Braille456,
            1456 => BraillePattern::Braille1456,
            2456 => BraillePattern::Braille2456,
            12456 => BraillePattern::Braille12456,
            3456 => BraillePattern::Braille3456,
            13456 => BraillePattern::Braille13456,
            23456 => BraillePattern::Braille23456,
            123456 => BraillePattern::Braille123456,

            _ => panic!("Invalid dot pattern."),
        }
    }

    pub fn to_unicode(&self) -> char {
        if let BraillePattern::BrailleNewline = self {
            'n'
        } else {
            unsafe { std::char::from_u32_unchecked(*self as u32 + 0x2800) }
        }
    }

    pub const fn has_lower_dots(&self) -> bool {
        (*self as u8 & 36) != 0
    }

    pub fn to_dots(&self) -> String {
        let mut res = String::with_capacity(6);
        for i in 1..=6 {
            if (*self as u8 & (1 << (i - 1))) > 0 {
                res.push_str(&i.to_string());
            }
        }
        res
    }
}

impl Add<BraillePattern> for BraillePattern {
    type Output = BraillePattern;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, rhs: BraillePattern) -> Self::Output {
        unsafe { std::mem::transmute(self as u8 | rhs as u8) }
    }
}

impl AddAssign<BraillePattern> for BraillePattern {
    fn add_assign(&mut self, rhs: BraillePattern) {
        *self = *self + rhs
    }
}
