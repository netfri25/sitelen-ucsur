use std::str::FromStr;

use crate::alt::Alt;
use crate::modifier::Modifier;
use crate::token::{Token, TokenKind};
use crate::word::Word;

const ALPHABET: &str = "aeijklmnopstuw";

pub struct Parser<'a> {
    input: &'a str,
    in_quotes: bool,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            in_quotes: false,
        }
    }

    pub fn next_token(&mut self) -> Option<Token<'a>> {
        // handle empty input
        if self.input.is_empty() {
            return None
        }

        if let Some(token) = self.parse_quote() {
            self.in_quotes = !self.in_quotes;
            self.consume(token.text().len());
            return Some(token)
        }

        let methods = [
            Self::parse_single_char,
            Self::parse_alternative,
            Self::parse_space,
            Self::parse_number,
            Self::parse_word,
            Self::parse_other,
        ];

        let token = methods.into_iter().find_map(|method| method(self))?;
        self.consume(token.text().len());
        Some(token)
    }

    fn consume(&mut self, count: usize) {
        self.input = &self.input[count..];
    }

    fn peek_char(&self) -> char {
        self.input.chars().next().unwrap_or_default()
    }

    // parse " to produce te or to
    fn parse_quote(&self) -> Option<Token<'a>> {
        if self.peek_char() != '"' {
            return None;
        }

        let quote = if self.in_quotes { Word::To } else { Word::Te };
        let kind = TokenKind::Word(quote);
        let token = Token::new("\"", kind);
        Some(token)
    }

    fn parse_single_char(&self) -> Option<Token<'a>> {
        let first = self.peek_char();

        // parse single character modifier
        let modifier = Modifier::from_char(first)?;
        let kind = TokenKind::Modifier(modifier);
        let token = Token::new(&self.input[..1], kind);
        Some(token)
    }

    fn parse_alternative(&self) -> Option<Token<'a>> {
        if self.peek_char() != '^' {
            return None;
        }

        let mut length = 1;

        let second = self.input.chars().nth(1).unwrap_or_default();

        let alt = if second.is_ascii_digit() {
            length += 1;
            let value = second as u8 - b'0';
            Alt::from_value(value).unwrap_or_default()
        } else {
            Alt::default()
        };

        let text = &self.input[..length];

        let token = Token::new(text, TokenKind::Alt(alt));
        Some(token)
    }

    fn parse_space(&self) -> Option<Token<'a>> {
        let leftover = self.input.trim_start_matches(' ');
        let count = self.input.len() - leftover.len();
        if count == 0 {
            return None;
        }

        let text = &self.input[..count];
        let kind = TokenKind::Space;
        let token = Token::new(text, kind);
        Some(token)
    }

    fn parse_number(&self) -> Option<Token<'a>> {
        let leftover = self.input.trim_start_matches(|c: char| c.is_ascii_digit());
        let count = self.input.len() - leftover.len();
        if count == 0 {
            return None;
        }

        let text = &self.input[..count];
        let kind = TokenKind::Number;
        let token = Token::new(text, kind);
        Some(token)
    }

    fn parse_word(&self) -> Option<Token<'a>> {
        let leftover = self.input.trim_start_matches(|c: char| c.is_alphabetic());
        let count = self.input.len() - leftover.len();
        if count == 0 {
            return None;
        }

        let text = &self.input[..count];

        let kind = Word::from_str(text)
            .map(TokenKind::Word)
            .unwrap_or_else(|_| {
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
        Some(token)
    }

    fn parse_other(&self) -> Option<Token<'a>> {
        // consume until next valid character
        let leftover = self.input.trim_start_matches(|c| !valid_char_token(c));
        let count = self.input.len() - leftover.len();
        let text = &self.input[..count];
        let token = Token::new(text, TokenKind::Other);
        Some(token)
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

fn valid_char_token(c: char) -> bool {
    ALPHABET.contains(c) || c.is_ascii_digit() || " ()[]{}+-_.:^".contains(c)
}
