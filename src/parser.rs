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
        Self { input, in_quotes: false }
    }

    pub fn next_token(&mut self) -> Token<'a> {
        // handle empty input
        if self.input.is_empty() {
            // TODO?: maybe it's better to use something like `Token::End`
            return Token::new("", TokenKind::Space);
        }

        let mut iter = self.input.chars();
        let first = iter.next().unwrap_or_default();
        let leftover = iter.as_str();

        // parse " to produce te or to
        if first == '"' {
            let quote = if self.in_quotes { Word::To } else { Word::Te };
            self.in_quotes = !self.in_quotes;
            self.input = leftover;
            return Token::new("\"", TokenKind::Word(quote));
        }

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

            let (text, leftover) = self.input.split_at(length);

            let token = Token::new(text, TokenKind::Alt(alt));
            self.input = leftover;
            return token;
        }

        // parse single character modifier
        let modifier = Modifier::from_char(first);
        if let Some(modifier) = modifier {
            let token = Token::new(&self.input[..1], TokenKind::Modifier(modifier));
            self.input = leftover;
            return token
        }

        // parse space
        let leftover = self.input.trim_start_matches(' ');
        let count = self.input.len() - leftover.len();
        if count > 0 {
            let text = &self.input[..count];
            let token = Token::new(text, TokenKind::Space);
            self.input = leftover;
            return token;
        }

        // parse number
        let leftover = self.input.trim_start_matches(|c: char| c.is_ascii_digit());
        let count = self.input.len() - leftover.len();
        if count > 0 {
            let text = &self.input[..count];
            self.input = leftover;
            return Token::new(text, TokenKind::Number);
        }

        // parse word
        let leftover = self.input.trim_start_matches(|c: char| c.is_alphabetic());
        let count = self.input.len() - leftover.len();
        if count > 0 {
            let text = &self.input[..count];

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
            self.input = leftover;
            return token;
        }

        // consume until next valid character
        let leftover = self.input.trim_start_matches(|c| !valid_char_token(c));
        let count = self.input.len() - leftover.len();
        let text = &self.input[..count];
        let token = Token::new(text, TokenKind::Other);

        self.input = leftover;
        token
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None
        }

        Some(self.next_token())
    }
}

fn valid_char_token(c: char) -> bool {
    c.is_alphabetic() || " ()[]{}+-_.:^".contains(c)
}

