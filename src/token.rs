use std::str::FromStr as _;

use crate::alt::Alt;
use crate::modifier::Modifier;
use crate::word::Word;

const ALPHABET: &str = "aeijklmnopstuw";

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
    Number(i32),

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

// token is either a "sitelen Lasina" or something else
pub fn next_token(input: &'_ str) -> (Token<'_>, &'_ str) {
    // handle empty input
    if input.is_empty() {
        // TODO?: maybe it's better to use something like `Token::End`
        return (Token::new("", TokenKind::Space), "");
    }

    let mut iter = input.chars();
    let first = iter.next().unwrap_or_default();
    let leftover = iter.as_str();

    // parse alternative character
    if first == '^' {
        let mut length = 1;

        let second = iter.next().unwrap_or_default();

        let alt = if second.is_ascii_digit() {
            length += 1;
            let value = second as u8 - b'0';
            Alt::from_value(value).unwrap_or_default()
        } else {
            Alt::default()
        };

        let (text, leftover) = input.split_at(length);

        let token = Token::new(text, TokenKind::Alt(alt));
        return (token, leftover);
    }

    // parse single character modifier
    let modifier = Modifier::from_char(first);
    if let Some(modifier) = modifier {
        let token = Token::new(&input[..1], TokenKind::Modifier(modifier));
        return (token, leftover);
    }

    // parse space
    let leftover = input.trim_start_matches(' ');
    let count = input.len() - leftover.len();
    if count > 0 {
        let text = &input[..count];
        let token = Token::new(text, TokenKind::Space);
        return (token, leftover);
    }

    // parse number
    let leftover = input.trim_start_matches(|c: char| c.is_ascii_digit());
    let count = input.len() - leftover.len();
    if count > 0 {
        let text = &input[..count];

        // TODO: maybe support very big numbers?
        let number = text.parse().expect("verified to be digits only");
        return (Token::new(text, TokenKind::Number(number)), leftover);
    }

    // parse word
    let leftover = input.trim_start_matches(|c: char| c.is_alphabetic());
    let count = input.len() - leftover.len();
    if count > 0 {
        let text = &input[..count];

        let kind = Word::from_str(text).map(TokenKind::Word).unwrap_or_else(|_| {
            if text
                .chars()
                .all(|c| ALPHABET.contains(c.to_ascii_lowercase()))
            {
                TokenKind::Lasina
            } else {
                TokenKind::Other
            }
        });

        let token = Token::new(text, kind);
        return (token, leftover);
    }

    // consume until next valid character
    let leftover = input.trim_start_matches(|c| !valid_char_token(c));
    let count = input.len() - leftover.len();
    let text = &input[..count];
    let token = Token::new(text, TokenKind::Other);

    (token, leftover)
}

fn valid_char_token(c: char) -> bool {
    c.is_alphabetic() || " ()[]{}+-_.:^".contains(c)
}

pub fn tokens(mut input: &'_ str) -> impl Iterator<Item = Token<'_>> {
    std::iter::from_fn(move || {
        if input.is_empty() {
            return None;
        }

        let token;
        (token, input) = next_token(input);
        Some(token)
    })
}
