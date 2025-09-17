use std::io;

use crate::lexer::Token;

pub fn write_tokens<'a>(
    out: &mut impl io::Write,
    iter: impl IntoIterator<Item = Token<'a>>
) -> io::Result<()> {
    let mut prev_is_word = false;
    for token in iter {
        // when the previous token was a word and the next token is a space, print one less token.
        // this is because words act kinda like characters, so there's no need to seperate them
        // with a redundant space.
        if prev_is_word && let Token::Space(spaces) = token {
            // print one less space
            let len = spaces.len().saturating_sub(1);
            write!(out, "{}", " ".repeat(len))?;
            continue
        }

        write!(out, "{}", token)?;

        prev_is_word = token.is_word();
    }

    Ok(())
}
