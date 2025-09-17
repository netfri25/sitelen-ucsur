use std::io;

mod show;
mod lexer;
mod word;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lines() {
        let Ok(line) = line else { break };
        let input = line.as_str();
        show::write_tokens(&mut io::stdout(), lexer::tokens(input)).unwrap();
        println!(); // newline + flush
    }
}
