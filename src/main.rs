use std::io;

use crate::lexer::Token;

mod show;
mod lexer;
mod word;
mod modifier;

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    while stdin.read_line(&mut line).is_ok() {
        if line.is_empty() {
            break
        }

        let mut prev_is_word = false;
        for token in lexer::tokens(&line) {
            match token {
                Token::Word(..) => prev_is_word = true,
                Token::Other(..) => prev_is_word = false,
                Token::Space(count) if !prev_is_word => {
                    print!("{}", " ".repeat(count));
                    continue
                }
                _ => {}
            }

            print!("{}", token);
        }

        line.clear();
    }
}
