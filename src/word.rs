include!(concat!(env!("OUT_DIR"), "/word.rs"));

use crate::lexer::Token;

enum Section {
    FullWord(Word),
    Dots(&'static str, u32),
}

impl Section {
    const fn text(&self) -> &'static str {
        match self {
            Self::FullWord(word) => word.as_lasina(),
            Self::Dots(text, ..) => text,
        }
    }

    fn weight(&self) -> u32 {
        match self {
            Self::FullWord(..) => 2,
            Self::Dots(_, dots, ..) => dots + 1,
        }
    }
}

pub fn find_minimal_word_construction(word: &str) -> Option<impl Iterator<Item = Token<'static>>> {
    find_minimal_word_construction_sections(word).map(|sections| {
        sections.into_iter().flat_map(|section| match section {
            Section::FullWord(word) => {
                std::iter::once(Token::Word(*word)).chain(std::iter::repeat_n(Token::Colon, 1))
            }
            Section::Dots(text, dots) => {
                let word = SECTION_TO_WORD[text];

                std::iter::once(Token::Word(word)).chain(std::iter::repeat_n(
                    Token::Dot,
                    usize::try_from(*dots).unwrap(),
                ))
            }
        })
    })
}

fn find_minimal_word_construction_sections(word: &str) -> Option<Vec<&'static Section>> {
    let word = word.to_lowercase();
    let word = word.as_str();

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct DpEntry {
        weight: u32,
        length: u32,
    }

    let mut dp = vec![
        DpEntry {
            weight: u32::MAX,
            length: u32::MAX
        };
        word.len() + 1
    ];

    let mut prev_index: Vec<usize> = vec![usize::MAX; word.len() + 1];
    let mut prev_section: Vec<Option<&'static Section>> = vec![None; word.len() + 1];

    dp[0] = DpEntry {
        weight: 0,
        length: 0,
    };

    for index in 0..dp.len() {
        if dp[index].weight == u32::MAX {
            continue;
        }

        for section in SECTIONS {
            let section_text = section.text();
            let next_index = index + section_text.len();

            if next_index > word.len() || !word[index..].starts_with(section_text) {
                continue;
            }

            let weight = dp[index].weight + section.weight();
            let length = dp[index].length + 1;
            let entry = DpEntry { weight, length };

            if entry < dp[next_index] {
                dp[next_index] = entry;
                prev_index[next_index] = index;
                prev_section[next_index] = Some(section);
            }
        }
    }

    if dp[word.len()].weight == u32::MAX {
        return None;
    }

    // capacity is worst case, where all sections are a single letter
    let mut output_sections = Vec::with_capacity(word.len());
    let mut index = word.len();

    while index > 0 {
        let Some(section) = prev_section[index] else {
            // no previous section. unable to construct
            return None;
        };

        output_sections.push(section);
        index = prev_index[index];

        // no next index
        if index == usize::MAX {
            return None;
        }
    }

    output_sections.reverse();
    Some(output_sections)
}
