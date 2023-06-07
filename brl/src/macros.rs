#[macro_export]
macro_rules! braille {
    ($($pattern:expr),*) => {
        brl::BrailleString::from_vec(smallvec::SmallVec::from_slice(&[
            $(brl::BraillePattern::from_dots($pattern),)*
        ])
        )
    }
}
