#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Alt(u8);

impl Alt {
    pub const fn from_value(value: u8) -> Option<Self> {
        if value > 9 { None } else { Some(Self(value)) }
    }

    pub fn as_lasina(self) -> &'static str {
        let start = self.0 * 2;
        &"^0^1^2^3^4^5^6^7^8^9"[start as usize..][..2]
    }

    pub fn from_sitelen(c: char) -> Option<Self> {
        let value = (c as u32)
            .checked_sub(0xfe00)
            .and_then(|v| u8::try_from(v).ok())?;
        Self::from_value(value)
    }

    pub const fn as_char(self) -> char {
        // SAFETY: it's fine trust me bro
        unsafe { char::from_u32_unchecked(self.0 as u32 + 0xfe00) }
    }
}

impl Default for Alt {
    fn default() -> Self {
        Self(1)
    }
}
