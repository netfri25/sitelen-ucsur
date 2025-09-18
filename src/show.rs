use std::collections::VecDeque;
use std::fmt;

use crate::lexer::Token;
use crate::modifier::Modifier;
use crate::word::Word;

struct Generator<'a, I: IntoIterator> {
    iter: I::IntoIter,
    scopes: Vec<Scope>,
    lookahead: VecDeque<Token<'a>>,

    // whether the last printed was a valid sitelen ucsur word
    prev_is_word: bool,
}

pub fn write_tokens<'a>(
    out: &mut impl fmt::Write,
    iter: impl IntoIterator<Item = Token<'a>>,
) -> fmt::Result {
    let mut g = Generator::new(iter);
    g.write_tokens(out)
}

impl<'a, I> Generator<'a, I>
where
    I: IntoIterator<Item = Token<'a>>,
{
    fn new(iter: I) -> Self {
        let iter = iter.into_iter();
        Self {
            iter,
            scopes: Default::default(),
            lookahead: Default::default(),
            prev_is_word: false,
        }
    }

    fn next_token(&mut self) -> Option<Token<'a>> {
        if !self.lookahead.is_empty() {
            self.lookahead.pop_front()
        } else {
            self.iter.next()
        }
    }

    fn peek(&mut self, index: usize) -> Option<&Token<'a>> {
        if self.lookahead.len() <= index {
            let to_read = index + 1 - self.lookahead.len();
            let to_read_iter = self.iter.by_ref().take(to_read);
            self.lookahead.extend(to_read_iter)
        }

        self.lookahead.get(index)
    }

    // remove from index at lookahead
    fn peek_remove(&mut self, index: usize) -> Option<Token<'a>> {
        self.lookahead.remove(index)
    }

    // returns `true` when "pi" was handled in a special way.
    // otherwise, returns `false` and "pi" should be handled as a regular word.
    fn handle_pi(&mut self, out: &mut impl fmt::Write) -> Result<bool, fmt::Error> {
        // search for a non-space character
        let mut i = 0;
        while self.peek(i).is_some_and(|t| t.is_space()) {
            i += 1
        }

        // if the following non-space token is not a LParen, then "pi" doesn't need any
        // special treatment.
        let Some(Token::LParen) = self.peek(i) else {
            return Ok(false);
        };

        // make sure to remove the LParen token since it's handled here
        self.peek_remove(i);

        // enter the "pi" scope
        self.push_scope(Scope::Pi);

        // write the special pi character
        write!(out, "{}", Modifier::StartOfLongPi.as_unicode_str())?;

        Ok(true)
    }

    fn write_spaces(&mut self, spaces: &'a str, out: &mut impl fmt::Write) -> fmt::Result {
        // figure out the correct "space" character
        let space_char = match self.scopes.last() {
            Some(Scope::Pi) => Modifier::CombiningLongPiExtension.as_unicode_str(),
            None => " ",
        };

        // when the previous token was a word and the next token is a space, print one less space.
        // this is because words act kinda like characters, so there's no need to seperate them
        // with a redundant space.
        let len = spaces.len().saturating_sub(self.prev_is_word as usize);
        write!(out, "{}", space_char.repeat(len))?;

        Ok(())
    }

    fn write_tokens(&mut self, out: &mut impl fmt::Write) -> fmt::Result {
        while let Some(token) = self.next_token() {
            if token == Token::Word(Word::Pi) && self.handle_pi(out)? {
                self.prev_is_word = true;
                continue
            }

            if token == Token::RParen && self.pop_scope(Scope::Pi) {
                write!(out, "{}", Modifier::EndOfLongGlyph.as_unicode_str())?;
                continue
            }

            if let Token::Space(spaces) = token {
                self.write_spaces(spaces, out)?;

                // don't register as the previous token
                continue;
            }

            write!(out, "{}", token)?;
            self.prev_is_word = !token.is_other();
        }

        Ok(())
    }

    fn push_scope(&mut self, scope: Scope) {
        self.scopes.push(scope)
    }

    fn pop_scope(&mut self, scope: Scope) -> bool {
        self.scopes.pop_if(|s| *s == scope).is_some()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Scope {
    Pi,
}
