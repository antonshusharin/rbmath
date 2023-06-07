use smallvec::SmallVec;
use std::{
    fmt::Display,
    ops::{AddAssign, Deref},
};

use crate::BraillePattern;

pub type InlineBrailleArray = [BraillePattern; 16];

#[derive(Default)]
pub struct BrailleString(SmallVec<InlineBrailleArray>);

impl BrailleString {
    pub fn new() -> BrailleString {
        BrailleString(SmallVec::new())
    }
    pub fn push(&mut self, chr: BraillePattern) {
        self.0.push(chr);
    }

    pub fn push_str(&mut self, str: &BrailleString) {
        self.0.extend_from_slice(str);
    }

    pub fn to_unicode(&self) -> String {
        self.0.iter().map(|c| c.to_unicode()).collect::<String>()
    }

    pub const fn from_vec(value: SmallVec<InlineBrailleArray>) -> BrailleString {
        BrailleString(value)
    }
}

impl AddAssign<&BrailleString> for BrailleString {
    fn add_assign(&mut self, rhs: &BrailleString) {
        self.push_str(rhs);
    }
}

impl AddAssign<BraillePattern> for BrailleString {
    fn add_assign(&mut self, rhs: BraillePattern) {
        self.push(rhs);
    }
}

impl Deref for BrailleString {
    type Target = [BraillePattern];
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl From<SmallVec<InlineBrailleArray>> for BrailleString {
    fn from(value: SmallVec<InlineBrailleArray>) -> Self {
        BrailleString(value)
    }
}

impl Display for BrailleString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_unicode().fmt(f)
    }
}

impl From<BraillePattern> for BrailleString {
    fn from(value: BraillePattern) -> Self {
        BrailleString(smallvec::smallvec![value])
    }
}
