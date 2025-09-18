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
        let mut output =  String::new();
        show::write_tokens(&mut output, lexer::tokens(input)).unwrap();
        println!("{}", output);
    }
}
