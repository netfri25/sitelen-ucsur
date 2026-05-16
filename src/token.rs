use crate::alt::Alt;
use crate::modifier::Modifier;
use crate::word::Word;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    // special characters (modifiers)
    Modifier(Modifier),

    // valid sitelen Lasina word
    Word(Word),

    // selection of an alternative character
    Alt(Alt),

    // non sitelen Lasina word but uses all alphabetical letters
    Lasina,

    // consecutive spaces
    Space,

    // number (will be converted to nnp)
    Number,

    // everything else
    Other,
}

impl TokenKind {
    pub fn from_sitelen(c: char) -> Option<Self> {
        if let Some(res) = Alt::from_sitelen(c) {
            return Some(Self::Alt(res));
        }

        if let Some(res) = Modifier::from_sitelen(c) {
            return Some(Self::Modifier(res));
        }

        if let Some(res) = Word::from_sitelen(c) {
            return Some(Self::Word(res));
        }

        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'a> {
    text: &'a str,
    kind: TokenKind,
}

impl<'a> Token<'a> {
    pub fn new(text: &'a str, kind: TokenKind) -> Self {
        Self { text, kind }
    }

    pub fn text(&self) -> &'a str {
        self.text
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn as_lasina(&self) -> &'a str {
        match self.kind {
            TokenKind::Modifier(modifier) => modifier.as_lasina(),
            TokenKind::Word(word) => word.as_lasina(),
            TokenKind::Alt(alt) => alt.as_lasina(),
            _ => self.text(),
        }
    }
}
