use std::env;
use std::io;

use crate::lexer::Token;

mod show;
mod lexer;
mod word;
mod modifier;

fn main() {
    let lasina_to_sitelen = parse_args();

    let mut stdout = io::stdout();
    let res = if lasina_to_sitelen {
        main_loop(&mut stdout, to_sitelen)
    } else {
        main_loop(&mut stdout, from_sitelen)
    };

    res.unwrap();
}

fn main_loop<F, O>(out: &mut O, mut conv: F) -> io::Result<()>
where
    F: FnMut(&mut O, &str) -> io::Result<()>,
    O: io::Write
{
    let stdin = io::stdin();
    let mut line = String::new();
    while stdin.read_line(&mut line).is_ok() {
        if line.is_empty() {
            break
        }

        conv(out, &line)?;
        out.flush()?;

        line.clear();
    }

    Ok(())
}

fn to_sitelen(out: &mut impl io::Write, input: &str) -> io::Result<()> {
    let mut prev_is_word = false;
    for token in lexer::tokens(input) {
        match token {
            Token::Word(..) => prev_is_word = true,
            Token::Other(..) => prev_is_word = false,
            Token::Space(spaces) if !prev_is_word => {
                write!(out, "{}", spaces)?;
                continue
            }
            _ => {}
        }

        write!(out, "{}", token)?;
    }

    Ok(())
}


fn from_sitelen(out: &mut impl io::Write, input: &str) -> io::Result<()> {
    for c in input.chars() {
        if let Some(token) = Token::from_sitelen(c) {
            write!(out, "{} ", token.as_literal())?;
        } else {
            write!(out, "{}", c)?;
        }
    }

    Ok(())
}

// returns true when it should parse lasina to sitelen
// return false when it should parse sitelen to lasina
fn parse_args() -> bool {
    let mut args = env::args();
    let program = args.next().unwrap_or_default();

    let Some(opt) = args.next() else {
        // if no arg is provided, by default try to parse lasina to sitelen
        return true;
    };

    match opt.as_str() {
        "from" => return true,
        "to" => return false,
        _ => {},
    }

    print_usage(&program);
    std::process::exit(1)
}

fn print_usage(program: &str) {
    eprintln!("Usage: {program} [from | to]")
}
