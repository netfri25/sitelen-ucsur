#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Modifier {
    StartOfCartouche = 0x0,
    EndOfCartouche = 0x1,
    // these aren't used (deprecated)
    // CombiningCartoucheExtension = 0x2,
    // StartOfLongPi = 0x3,
    // CombiningLongPiExtension = 0x4,
    StackingJoiner = 0x5,
    ScalingJoiner = 0x6,
    StartOfLongGlyph = 0x7,
    EndOfLongGlyph = 0x8,
    CombiningLongGlyphExtension = 0x9,
    StartOfReverseLongGlyph = 0xa,
    EndOfReverseLongGlyph = 0xb,
    MiddleDot = 0xc,
    Colon = 0xd,
}

impl Modifier {
    // returns the unicode representation of the modifier
    pub const fn as_sitelen(self) -> char {
        unsafe {
            // SAFETY: since the values of the Modifier enum are carefully picked, we know for a
            // fact that it will be a valid unicode value, so it can be `unchecked`
            char::from_u32_unchecked(0xf1990 + self as u32)
        }
    }
}
