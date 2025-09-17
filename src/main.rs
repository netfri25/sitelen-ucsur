use std::fmt::Write;

mod word;
mod lexer;

fn main() {
    let stdin = std::io::stdin();
    for line in stdin.lines() {
        let Ok(line) = line else {
            break
        };

        let mut output = String::new();
        let mut input = line.trim();
        let mut token;

        while !input.is_empty() {
            (token, input) = lexer::next_token(input);
            write!(&mut output, "{}", token).unwrap();
        }

        println!("{}", output);
    }
}
