// build.rs

use std::env;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("word.rs");
    let mut output = BufWriter::new(File::create(dest_path).unwrap());
    generate(&mut output).unwrap();
    println!("cargo::rerun-if-changed=build.rs");
}

fn generate(out: &mut impl Write) -> io::Result<()> {
    writeln!(out, "#[repr(u8)]")?;
    writeln!(out, "#[derive(Debug, Clone, Copy, PartialEq, Eq)]")?;
    writeln!(out, "pub enum Word {{")?;
    for (i, word) in WORDS.iter().enumerate() {
        let word = capitalize(word);
        writeln!(out, "    {word} = 0x{i:02x},")?;
    }
    writeln!(out, "}}")?;


    writeln!(out)?;

    writeln!(out, "impl Word {{")?;

    write!(out, "    const SITELEN_CHAR: [char; {WORDS_COUNT}] = [")?;
    for i in 0..WORDS.len() as u32 {
        write!(out, "'{}',", char::from_u32(UNICODE_OFFSET + i).unwrap())?;
    }
    writeln!(out, "];")?;

    writeln!(out)?;

    write!(out, "    const LASINA_WORD: [&str; {WORDS_COUNT}] = [")?;
    for word in WORDS {
        write!(out, "\"{word}\",")?;
    }
    writeln!(out, "];")?;

    writeln!(out)?;

    writeln!(out, "    pub fn as_sitelen(self) -> char {{")?;
    writeln!(out, "        Self::SITELEN_CHAR[self as usize]")?;
    writeln!(out, "    }}")?;
    writeln!(out)?;
    writeln!(out, "    pub fn as_lasina(self) -> &'static str {{")?;
    writeln!(out, "        Self::LASINA_WORD[self as usize]")?;
    writeln!(out, "    }}")?;

    writeln!(out)?;

    writeln!(out, "    pub fn from_sitelen(c: char) -> Option<Self> {{")?;
    writeln!(out, "        let value = c as u32;")?;
    writeln!(out, "        ({UNICODE_OFFSET}..{UNICODE_OFFSET} + {WORDS_COUNT})")?;
    writeln!(out, "            .contains(&value)")?;
    writeln!(out, "            .then(|| {{")?;
    writeln!(out, "                // SAFETY: we transmute to the enum after it has been verified to be in the")?;
    writeln!(out, "                // range of the enum. same for the subtraction, since the check applies before")?;
    writeln!(out, "                // evaluation the subtraction, and the same for the casting of the u8.")?;
    writeln!(out, "                unsafe {{ std::mem::transmute(value.unchecked_sub({UNICODE_OFFSET}) as u8) }}")?;
    writeln!(out, "            }})")?;
    writeln!(out, "    }}")?;

    writeln!(out, "}}")?;

    writeln!(out)?;

    // TODO: change this to a `.position()` on an iter
    writeln!(out, "impl std::str::FromStr for Word {{")?;
    writeln!(out, "    type Err = ();")?;
    writeln!(out)?;
    writeln!(out, "    fn from_str(s: &str) -> Result<Self, ()> {{")?;
    writeln!(out, "        match s {{")?;
    for word in WORDS {
        let cap = capitalize(word);
        writeln!(out, "            \"{word}\" => Ok(Self::{cap}),")?;
    }
    writeln!(out, "            _ => Err(()),")?;
    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;

    Ok(())
}

fn capitalize(input: &str) -> String {
    let mut upper = input.to_uppercase();
    upper[1..].make_ascii_lowercase();
    upper
}

const UNICODE_OFFSET: u32 = 0xf1900;

const WORDS_COUNT: usize = 137;

#[rustfmt::skip]
const WORDS: [&str; WORDS_COUNT] = ["a", "akesi", "ala", "alasa", "ale", "anpa", "ante", "anu", "awen", "e", "en", "esun", "ijo", "ike", "ilo", "insa", "jaki", "jan", "jelo", "jo", "kala", "kalama", "kama", "kasi", "ken", "kepeken", "kili", "kiwen", "ko", "kon", "kule", "kulupu", "kute", "la", "lape", "laso", "lawa", "len", "lete", "li", "lili", "linja", "lipu", "loje", "lon", "luka", "lukin", "lupa", "ma", "mama", "mani", "meli", "mi", "mije", "moku", "moli", "monsi", "mu", "mun", "musi", "mute", "nanpa", "nasa", "nasin", "nena", "ni", "nimi", "noka", "o", "olin", "ona", "open", "pakala", "pali", "palisa", "pan", "pana", "pi", "pilin", "pimeja", "pini", "pipi", "poka", "poki", "pona", "pu", "sama", "seli", "selo", "seme", "sewi", "sijelo", "sike", "sin", "sina", "sinpin", "sitelen", "sona", "soweli", "suli", "suno", "supa", "suwi", "tan", "taso", "tawa", "telo", "tenpo", "toki", "tomo", "tu", "unpa", "uta", "utala", "walo", "wan", "waso", "wawa", "weka", "wile", "namako", "kin", "oko", "kipisi", "leko", "monsuta", "tonsi", "jasima", "kijetesantakalu", "soko", "meso", "epiku", "kokosila", "lanpan", "n", "misikeke", "ku"];
