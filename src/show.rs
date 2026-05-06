use std::fmt::{self, Write as _};

use crate::token::Token;
use crate::modifier::Modifier;
use crate::word::find_minimal_word_construction;

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Modifier(modifier) => f.write_char(modifier.as_sitelen()),
            Token::Word(word) => f.write_char(word.as_sitelen()),
            Token::Lasina(s) => {
                f.write_char(Modifier::StartOfCartouche.as_sitelen())?;
                construct_name(s, f)?;
                f.write_char(Modifier::EndOfCartouche.as_sitelen())
            }
            Token::Other(other) => f.write_str(other),
            Token::Space(spaces) => f.write_str("\u{3000}".repeat(spaces.len() / 2).as_str()),
        }
    }
}

fn construct_name(s: &str, f: &mut fmt::Formatter) -> fmt::Result {
    let Some(mut tokens) = find_minimal_word_construction(s) else {
        return construct_name_simple(s, f);
    };

    tokens.try_for_each(|token| fmt::Display::fmt(&token, f))
}

fn construct_name_simple(s: &str, f: &mut fmt::Formatter) -> fmt::Result {
    s.chars().try_for_each(|c| {
        let c = match c.to_ascii_lowercase() {
            'a' => '󱤆',
            'e' => '󱤉',
            'i' => '󱤏',
            'j' => '󱤒',
            'k' => '󱤕',
            'l' => '󱤩',
            'm' => '󱤱',
            'n' => '󱥀',
            'o' => '󱥇',
            'p' => '󱥈',
            's' => '󱥡',
            't' => '󱥩',
            'u' => '󱥱',
            'w' => '󱥵',
            c => c,
        };

        f.write_char(c)
    })
}
