// we don't represent the modifier as directly the values it maps to, since they're 32 bit values
// and it's an overkill for this enum to be a 32 bit value.

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // don't warn about the unused deprecated variants
pub enum Modifier {
    StartOfCartouche = 0x0,
    EndOfCartouche = 0x1,

    CombiningCartoucheExtension = 0x2, // deprecated
    StartOfLongPi = 0x3,               // deprecated
    CombiningLongPiExtension = 0x4,    // deprecated

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

const MODIFIER_START: u32 = 0xf1990;

impl Modifier {
    // returns the unicode representation of the modifier
    pub const fn as_sitelen(self) -> char {
        unsafe {
            // SAFETY: since the values of the Modifier enum are carefully picked, we know for a
            // fact that it will be a valid unicode value, so it can be `unchecked`
            char::from_u32_unchecked(MODIFIER_START + self as u32)
        }
    }

    pub const fn from_sitelen(c: char) -> Option<Self> {
        let value = c as u32;
        if value >= MODIFIER_START && value <= MODIFIER_START + Self::Colon as u32 {
            let target_variant = c as u32 - MODIFIER_START;
            // SAFETY: value is verified to be in the correct range, and can be translated by a
            // simple subtraction operation
            let variant = unsafe { std::mem::transmute::<u8, Self>(target_variant as _) };
            Some(variant)
        } else {
            None
        }
    }

    pub const fn as_lasina(self) -> &'static str {
        match self {
            Self::StartOfLongGlyph => "(",
            Self::EndOfLongGlyph => ")",
            Self::StartOfCartouche => "[",
            Self::EndOfCartouche => "]",
            Self::StartOfReverseLongGlyph => "{",
            Self::EndOfReverseLongGlyph => "}",
            Self::ScalingJoiner => "+",
            Self::StackingJoiner => "-",
            Self::CombiningLongGlyphExtension => "_",
            Self::MiddleDot => ".",
            Self::Colon => ":",

            // deprecated and not really used
            Self::CombiningCartoucheExtension => "_",
            Self::StartOfLongPi => "(",
            Self::CombiningLongPiExtension => "_",
        }
    }

    pub const fn from_char(c: char) -> Option<Self> {
        Some(match c {
            '(' => Self::StartOfLongGlyph,
            ')' => Self::EndOfLongGlyph,
            '[' => Self::StartOfCartouche,
            ']' => Self::EndOfCartouche,
            '{' => Self::StartOfReverseLongGlyph,
            '}' => Self::EndOfReverseLongGlyph,
            '+' => Self::ScalingJoiner,
            '-' => Self::StackingJoiner,
            '_' => Self::CombiningLongGlyphExtension,
            '.' => Self::MiddleDot,
            ':' => Self::Colon,
            _ => return None,
        })
    }
}
