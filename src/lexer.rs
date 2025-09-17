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
    let leftover = input.trim_matches(' ');
    let spaces = input.len() - leftover.len();
    if spaces > 0 {
        let token = Token::Space(&input[..spaces]);
        return (token, leftover);
    }

    // since spaces == 0, input.len == leftover.len, thus input == leftover

    let count = input.chars().take_while(|c| ALPHABET.contains(c)).count();
    if count > 0 {
        let (text, input) = input.split_at(count);
        let token = Word::from_str(text)
            .map(Token::Word)
            .unwrap_or(Token::Other(text));

        return (token, input);
    }

    let count = input
        .chars()
        .take_while(|c| !ALPHABET.contains(c) && *c != ' ')
        .count();

    let (text, input) = input.split_at(count);
    let token = Token::Other(text);

    (token, input)
}
