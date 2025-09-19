use std::fmt::{self, Write as _};

use crate::lexer::Token;
use crate::modifier::Modifier;

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::LParen
            | Token::RParen
            | Token::LBrack
            | Token::RBrack
            | Token::LBrace
            | Token::RBrace
            | Token::Plus
            | Token::Minus
            | Token::Underscore
            | Token::Dot
            | Token::Colon => {
                let modifier = TOKEN_MODIFIER
                    .iter()
                    .find(|(t, _)| t == self)
                    .expect("modifier for token exists")
                    .1;
                f.write_char(modifier.as_sitelen())
            }

            Token::Word(word) => f.write_char(word.as_sitelen()),
            Token::Other(s) => f.write_str(s),
            Token::Space(spaces) => f.write_str("\u{3000}".repeat(spaces.len() / 2).as_str()),
        }
    }
}

pub const TOKEN_MODIFIER: [(Token, Modifier); 11] = [
    (Token::LParen, Modifier::StartOfLongGlyph),
    (Token::RParen, Modifier::EndOfLongGlyph),
    (Token::LBrack, Modifier::StartOfCartouche),
    (Token::RBrack, Modifier::EndOfCartouche),
    (Token::LBrace, Modifier::StartOfReverseLongGlyph),
    (Token::RBrace, Modifier::EndOfReverseLongGlyph),
    (Token::Plus, Modifier::ScalingJoiner),
    (Token::Minus, Modifier::StackingJoiner),
    (Token::Underscore, Modifier::CombiningLongGlyphExtension),
    (Token::Dot, Modifier::MiddleDot),
    (Token::Colon, Modifier::Colon),
];
