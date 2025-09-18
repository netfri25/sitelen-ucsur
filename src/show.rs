use std::fmt;

use crate::lexer::Token;
use crate::modifier::Modifier;

pub fn write_tokens<'a>(
    out: &mut impl fmt::Write,
    iter: impl IntoIterator<Item = Token<'a>>,
) -> fmt::Result {
    let mut prev_is_word = false;

    iter.into_iter().try_for_each(|token| {
        match token {
            Token::LParen => out.write_str(Modifier::StartOfLongGlyph.as_unicode_str()),
            Token::RParen => out.write_str(Modifier::EndOfLongGlyph.as_unicode_str()),
            Token::LBrack => out.write_str(Modifier::StartOfCartouche.as_unicode_str()),
            Token::RBrack => out.write_str(Modifier::EndOfCartouche.as_unicode_str()),
            Token::LBrace => out.write_str(Modifier::StartOfReverseLongGlyph.as_unicode_str()),
            Token::RBrace => out.write_str(Modifier::EndOfReverseLongGlyph.as_unicode_str()),

            Token::Word(word) => {
                prev_is_word = true;
                out.write_str(word.as_unicode_str())
            }

            Token::Space(spaces) => {
                let len = spaces.len().saturating_sub(prev_is_word as usize);
                out.write_str(" ".repeat(len).as_str())
            }

            Token::Other(s) => {
                prev_is_word = false;
                out.write_str(s)
            }
        }
    })
}
