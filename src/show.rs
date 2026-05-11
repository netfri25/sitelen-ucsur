use std::fmt::{self, Write as _};

use crate::modifier::Modifier;
use crate::token::Token;
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
            Token::Number(n) => {
                // TODO: maybe support very big numbers?
                let number = n.parse().expect("number is known to be digit-only");
                construct_number_nnp(number, f)
            }
            Token::Alt(alt) => f.write_char(alt.as_char()),
            Token::Other(other) => f.write_str(other),
            Token::Space(spaces) => f.write_str("\u{3000}".repeat(spaces.len() / 2).as_str()),
        }
    }
}

fn construct_number_nnp(mut n: i32, f: &mut fmt::Formatter) -> fmt::Result {
    if n == 0 { return f.write_str("󱤂"); }

    // nanpa li suli la o kipisi e ona kepeken nimi "ale"
    if n >= 100 {
        construct_number_nnp(n / 100, f)?;
        f.write_str("󱤄")?;
        n %= 100;
    }

    while n >= 20 { f.write_str("󱤼")?; n -= 20; }
    while n >= 5 { f.write_str("󱤭")?; n -= 5; }
    while n >= 2 { f.write_str("󱥮")?; n -= 2; }
    while n >= 1 { f.write_str("󱥳")?; n -= 1; }

    return Ok(());
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
