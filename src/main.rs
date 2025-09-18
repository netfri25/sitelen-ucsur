use std::io;

mod show;
mod lexer;
mod word;
mod modifier;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lines() {
        let Ok(line) = line else { break };
        let input = line.as_str();
        for token in lexer::tokens(input) {
            print!("{}", token);
        }
        println!()
    }
}
