use std::fmt;
use std::str::FromStr as _;

use crate::word::Word;

const ALPHABET_LETTERS: usize = 14;
const ALPHABET: [char; ALPHABET_LETTERS] = [
    'a', 'e', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 's', 't', 'u', 'w',
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'a> {
    Word(Word),
    Space(&'a str), // consecutive spaces
    Other(&'a str),
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Word(word) => f.write_str(word.as_unicode_str()),
            Token::Other(s) => f.write_str(s),
            Token::Space(s) => f.write_str(s),
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

    // parse space
    let leftover = input.trim_start_matches(' ');
    let count = input.len() - leftover.len();
    if count > 0 {
        let token = Token::Space(&input[..count]);
        return (token, leftover);
    }

    // parse word
    let leftover = input.trim_start_matches(|c| ALPHABET.contains(&c));
    let count = input.len() - leftover.len();
    if count > 0 {
        let text = &input[..count];

        let token = Word::from_str(text)
            .map(Token::Word)
            .unwrap_or(Token::Other(text));

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
    ALPHABET.contains(&c) || c == ' '
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
