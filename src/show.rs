use std::fmt;

use crate::lexer::Token;
use crate::modifier::Modifier;

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::LParen => f.write_str(Modifier::StartOfLongGlyph.as_unicode_str()),
            Token::RParen => f.write_str(Modifier::EndOfLongGlyph.as_unicode_str()),
            Token::LBrack => f.write_str(Modifier::StartOfCartouche.as_unicode_str()),
            Token::RBrack => f.write_str(Modifier::EndOfCartouche.as_unicode_str()),
            Token::LBrace => f.write_str(Modifier::StartOfReverseLongGlyph.as_unicode_str()),
            Token::RBrace => f.write_str(Modifier::EndOfReverseLongGlyph.as_unicode_str()),
            Token::Plus => f.write_str(Modifier::ScalingJoiner.as_unicode_str()),
            Token::Minus => f.write_str(Modifier::StackingJoiner.as_unicode_str()),
            Token::Underscore => {
                f.write_str(Modifier::CombiningLongGlyphExtension.as_unicode_str())
            }
            Token::Dot => f.write_str(Modifier::MiddleDot.as_unicode_str()),
            Token::Colon => f.write_str(Modifier::Colon.as_unicode_str()),
            Token::Word(word) => f.write_str(word.as_unicode_str()),
            Token::Other(s) => f.write_str(s),
            Token::Space(count) => f.write_str("\u{3000}".repeat(count / 2).as_str()),
        }
    }
}
