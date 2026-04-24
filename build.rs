// build.rs

use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::sync::LazyLock;

const WORDS_FILE_CONTENT: &str = include_str!("./dict.txt");

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
    for (i, (_, word)) in words_iter().enumerate() {
        let word = capitalize(word);
        writeln!(out, "    {word} = 0x{i:02x},")?;
    }
    writeln!(out, "}}")?;

    writeln!(out)?;

    writeln!(out, "impl Word {{")?;

    write!(out, "    const SITELEN_CHAR: &[char] = &[")?;
    for (value, _) in words_iter() {
        write!(out, "'{}',", char::from_u32(value).unwrap())?;
    }
    writeln!(out, "];")?;

    writeln!(out)?;

    write!(out, "    const LASINA_WORD: &[&str] = &[")?;
    for (_, word) in words_iter() {
        write!(out, "\"{word}\",")?;
    }
    writeln!(out, "];")?;

    writeln!(out)?;

    writeln!(out, "    pub const fn as_sitelen(self) -> char {{")?;
    writeln!(out, "        Self::SITELEN_CHAR[self as usize]")?;
    writeln!(out, "    }}")?;
    writeln!(out)?;
    writeln!(out, "    pub const fn as_lasina(self) -> &'static str {{")?;
    writeln!(out, "        Self::LASINA_WORD[self as usize]")?;
    writeln!(out, "    }}")?;

    writeln!(out)?;

    // SAFETY: we transmute to the enum after it has been
    // verified to be in the range of the enum. same for
    // the subtraction, since the check applies before
    // evaluation the subtraction, and the same for
    // the casting of the u8.
    writeln!(out, "    pub fn from_sitelen(c: char) -> Option<Self> {{")?;
    writeln!(out, "        Self::SITELEN_CHAR")?;
    writeln!(out, "            .iter()")?;
    writeln!(out, "            .position(|&x| x == c)")?;
    writeln!(out, "            .map(|i| unsafe {{")?;
    writeln!(out, "                std::mem::transmute(i as u8)")?;
    writeln!(out, "            }})")?;
    writeln!(out, "    }}")?;

    writeln!(out, "}}")?;

    writeln!(out)?;

    writeln!(out, "impl std::str::FromStr for Word {{")?;
    writeln!(out, "    type Err = ();")?;
    writeln!(out)?;
    writeln!(out, "    fn from_str(s: &str) -> Result<Self, ()> {{")?;
    writeln!(out, "        match s {{")?;
    for (_, word) in words_iter() {
        let cap = capitalize(word);
        writeln!(out, "            \"{word}\" => Ok(Self::{cap}),")?;
    }
    writeln!(out, "            _ => Err(()),")?;
    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;
    writeln!(out)?;

    let mut section_to_words: BTreeMap<&'static str, &'static str> = Default::default();

    for (_, word) in words_iter() {
        for section in sections_from_word(word) {
            section_to_words.entry(section.text()).or_insert(word);
        }
    }

    writeln!(out, "static SECTION_TO_WORD: phf::Map<&str, Word> = phf::phf_map! {{")?;
    for (key, value) in section_to_words {
        writeln!(out, "    \"{}\" => Word::{},", key, capitalize(value))?;
    }
    writeln!(out, "}};")?;
    writeln!(out)?;

    writeln!(out, "static SECTIONS: &[Section] = &[")?;
    for (_, word) in words_iter() {
        for section in sections_from_word(word) {
            match section {
                Section::FullWord(text) => {
                    writeln!(out, "    Section::FullWord(Word::{}),", capitalize(text))?
                }
                Section::Dots(text, dots) => {
                    writeln!(out, "    Section::Dots(\"{}\", {}),", text, dots)?
                }
            }
        }
    }
    writeln!(out, "];")?;
    writeln!(out)?;

    Ok(())
}

fn capitalize(input: &str) -> String {
    let mut upper = input.to_uppercase();
    upper[1..].make_ascii_lowercase();
    upper
}

fn words_iter() -> impl Iterator<Item = (u32, &'static str)> + 'static {
    static WORDS: LazyLock<Vec<(u32, &'static str)>> = LazyLock::new(|| {
        WORDS_FILE_CONTENT
            .lines()
            .map(|line| {
                let mut iter = line.split_whitespace();
                let value = iter
                    .next()
                    .and_then(|value_str| {
                        u32::from_str_radix(value_str.trim_start_matches("0x"), 16).ok()
                    })
                    .unwrap();
                let name = iter.next().unwrap();
                (value, name)
            })
            .collect()
    });

    WORDS.iter().copied()
}

struct MorasIter<'a> {
    prev: usize,
    input: &'a str,
}

impl<'a> Iterator for MorasIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.prev == self.input.len() {
            return None;
        }

        let index = self.input[self.prev..]
            .find(|c| "aeioun".contains(c))
            .map(|offset| self.prev + offset + 1)
            .unwrap_or(self.input.len());

        let mora = &self.input[..index];
        self.prev = index;
        if self.prev == self.input.len() {
            // if it's the full word, don't yield it
            return None;
        }

        Some(mora)
    }
}

fn split_moras(input: &str) -> impl Iterator<Item = &str> {
    MorasIter { prev: 0, input }
}

fn word_prefixes(input: &str) -> impl Iterator<Item = &str> {
    let first_letter_iter = std::iter::once(&input[..1]);

    if "aeioun".contains(&input[..1]) {
        // if the first letter is the letter that the moras are split on, it means that the split
        // moras will have a single letter as the first value of the iterator. we don't want that,
        // since we already chain it with an iterator of a single letter, so we skip a single value.
        first_letter_iter.chain(split_moras(input).skip(1))
    } else {
        // we have to use `skip(0)` since the return type of the iterator must match!
        // and since the `skip` method on the iterator returns a new type, we have to also use
        // it in the other branch.
        #[allow(clippy::iter_skip_zero)]
        first_letter_iter.chain(split_moras(input).skip(0))
    }
}

fn sections_from_word<'a>(word: &'a str) -> impl Iterator<Item = Section<'a>> {
    std::iter::once(Section::FullWord(word)).chain(
        word_prefixes(word)
            .enumerate()
            .map(|(i, prefix)| Section::Dots(prefix, i)),
    )
}

pub enum Section<'a> {
    FullWord(&'a str),
    Dots(&'a str, usize),
}

impl<'a> Section<'a> {
    pub fn text(&self) -> &'a str {
        match self {
            Self::FullWord(text) => text,
            Self::Dots(text, ..) => text,
        }
    }
}
