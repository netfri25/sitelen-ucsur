#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Modifier {
    StartOfCartouche = 0x0,
    EndOfCartouche = 0x1,
    // these aren't used
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
        UNICODE_TABLE[self as usize]
    }
}

pub const UNICODE_TABLE: [char; 14] = [
    '\u{f1990}',
    '\u{f1991}',
    '\u{f1992}',
    '\u{f1993}',
    '\u{f1994}',
    '\u{f1995}',
    '\u{f1996}',
    '\u{f1997}',
    '\u{f1998}',
    '\u{f1999}',
    '\u{f199a}',
    '\u{f199b}',
    '\u{f199c}',
    '\u{f199d}',
];
