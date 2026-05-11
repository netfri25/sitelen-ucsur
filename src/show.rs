use std::fmt::{self, Write as _};

use crate::modifier::Modifier;
use crate::token::{Token, TokenKind};
use crate::word::{Word, find_minimal_word_construction};

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind() {
            TokenKind::Modifier(modifier) => f.write_char(modifier.as_sitelen()),
            TokenKind::Word(word) => f.write_char(word.as_sitelen()),
            TokenKind::Lasina => {
                f.write_char(Modifier::StartOfCartouche.as_sitelen())?;
                construct_name(self.text(), f)?;
                f.write_char(Modifier::EndOfCartouche.as_sitelen())
            }
            TokenKind::Number => construct_number_nnp(self.text(), f),
            TokenKind::Alt(alt) => f.write_char(alt.as_char()),
            TokenKind::Other => f.write_str(self.text()),
            TokenKind::Space => f.write_str("\u{3000}".repeat(self.text().len() / 2).as_str()),
        }
    }
}

fn construct_number_nnp(text: &str, f: &mut fmt::Formatter) -> fmt::Result {
    let text = text.trim_start_matches('0');

    if text.is_empty() {
        return f.write_char(Word::Ala.as_sitelen());
    }

    // if even - second slice starts from index 2
    // if odd  - second slice starts from index 1
    let start_offset = 2 - text.len() % 2;

    let (first_slice, mut rest) = text.split_at(start_offset);

    let first_number = first_slice.parse().unwrap();

    // special case for numbers that start with the digit 1:
    // if the first number is one, don't type it, unless there are no more sections (which means
    // that the number is simply 1)
    // just `ale ...`
    if first_number != 1 || rest.len() < 2 {
        construct_small_number_nnp(first_number, f)?;
    }

    let chunks = std::iter::from_fn(|| {
        if rest.is_empty() {
            return None
        }
        let chunk;
        (chunk, rest) = rest.split_at(2);
        Some(chunk)
    });

    // nanpa li suli la o kipisi e ona kepeken nimi "ale"
    chunks
        .into_iter()
        .try_for_each(|n| {
            f.write_char(Word::Ale.as_sitelen())?;
            let n = n.parse().unwrap();
            construct_small_number_nnp(n, f)
        })
}

// should get a number in the range [0, 100) (in rust, 0..100)
fn construct_small_number_nnp(mut n: i32, f: &mut fmt::Formatter) -> fmt::Result {
    while n >= 20 { f.write_str("󱤼")?; n -= 20; }
    while n >= 5 { f.write_str("󱤭")?; n -= 5; }
    while n >= 2 { f.write_str("󱥮")?; n -= 2; }
    while n >= 1 { f.write_str("󱥳")?; n -= 1; }

    Ok(())
}

fn construct_name(s: &str, f: &mut fmt::Formatter) -> fmt::Result {
    let Some(tokens) = find_minimal_word_construction(s) else {
        return construct_name_simple(s, f);
    };

    tokens
        .into_iter()
        .try_for_each(|token| fmt::Display::fmt(&token, f))
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
