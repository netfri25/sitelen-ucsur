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
        let input = line.as_str();

        for token in lexer::tokens(input) {
            dbg!(token);
            write!(&mut output, "{}", token).unwrap();
        }

        println!("{}", output);
    }
}
