use std::str::FromStr as _;

use crate::alt::Alt;
use crate::modifier::Modifier;
use crate::word::Word;

const ALPHABET: &str = "aeijklmnopstuw";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'a> {
    // special characters (modifiers)
    Modifier(Modifier),

    // valid sitelen Lasina word
    Word(Word),

    // selection of an alternative character
    Alt(Alt),

    // non sitelen Lasina word but uses all alphabetical letters
    Lasina(&'a str),

    // consecutive spaces
    Space(&'a str),

    // number (will be converted to nnp)
    Number(i32),

    // everything else
    Other(&'a str),
}

impl<'a> Token<'a> {
    pub fn from_sitelen(c: char) -> Option<Self> {
        if let Some(res) = Alt::from_sitelen(c) {
            return Some(Token::Alt(res));
        }

        if let Some(res) = Modifier::from_sitelen(c) {
            return Some(Token::Modifier(res));
        }

        if let Some(res) = Word::from_sitelen(c) {
            return Some(Token::Word(res));
        }

        None
    }

    pub fn as_literal(&self) -> &'a str {
        match self {
            Token::Modifier(modifier) => modifier.as_lasina(),
            Token::Word(word) => word.as_lasina(),
            Token::Alt(alt) => alt.as_lasina(),
            Token::Space(spaces) => spaces,
            Token::Lasina(word) => word,
            Token::Number(_n) => unreachable!(),
            Token::Other(other) => other,
        }
    }
}

// token is either a "sitelen Lasina" or something else
pub fn next_token(input: &'_ str) -> (Token<'_>, &'_ str) {
    // handle empty input
    if input.is_empty() {
        // TODO?: maybe it's better to use something like `Token::End`
        return (Token::Space(""), "");
    }

    let mut iter = input.chars();
    let first = iter.next().unwrap_or_default();
    let leftover = iter.as_str();

    // parse alternative character
    if first == '^' {
        let mut leftover = leftover;

        let second = iter.next().unwrap_or_default();

        let alt = if second.is_ascii_digit() {
            leftover = &leftover[1..];
            let value = second as u8 - b'0';
            Alt::from_value(value).unwrap_or_default()
        } else {
            Alt::default()
        };

        let token = Token::Alt(alt);
        return (token, leftover);
    }

    // parse single character modifier
    let modifier = Modifier::from_char(first);
    if let Some(modifier) = modifier {
        let token = Token::Modifier(modifier);
        return (token, leftover);
    }

    // parse space
    let leftover = input.trim_start_matches(' ');
    let count = input.len() - leftover.len();
    if count > 0 {
        let token = Token::Space(&input[..count]);
        return (token, leftover);
    }

    // parse number
    let leftover = input.trim_start_matches(|c: char| c.is_ascii_digit());
    let count = input.len() - leftover.len();
    if count > 0 {
        let text = &input[..count];
        return (Token::Number(text.parse::<i32>().unwrap()), leftover);
    }

    // parse word
    let leftover = input.trim_start_matches(|c: char| c.is_alphabetic());
    let count = input.len() - leftover.len();
    if count > 0 {
        let text = &input[..count];

        let token = Word::from_str(text).map(Token::Word).unwrap_or_else(|_| {
            if text
                .chars()
                .all(|c| ALPHABET.contains(c.to_ascii_lowercase()))
            {
                Token::Lasina(text)
            } else {
                Token::Other(text)
            }
        });

        return (token, leftover);
    }

    // consume until next valid character
    let leftover = input.trim_start_matches(|c| !valid_char_token(c));
    let count = input.len() - leftover.len();
    let text = &input[..count];
    let token = Token::Other(text);

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
