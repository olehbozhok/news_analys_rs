use rayon::prelude::*;
use whatlang::{detect, Lang};

mod get_data;
pub use get_data::*;

pub mod summarizer;

pub use stemmer_uk::stem_word;

pub struct StemWord {
    pub word: String,
    pub stem: String,
}

pub struct StemSentence {
    pub origin_id: usize,
    pub stem: Vec<StemWord>,
}

pub fn prepare_sentences(sent: Vec<&str>, lang: Lang) -> Vec<StemSentence> {
    sent.into_par_iter()
        .enumerate()
        .filter(|(_i, q)| {
            let info = detect(q).unwrap();
            info.lang() == lang && info.is_reliable()
        })
        .map(|(i, sen)| StemSentence {
            origin_id: i,
            stem: stem_sentence(sen.into()),
        })
        .collect()
}

fn stem_sentence(sentence: String) -> Vec<StemWord> {
    let words = summarizer::split_into_words(sentence.as_str());
    words
        .into_iter()
        .map(|s| stem_word_struct(s.into()))
        .collect()
}

fn stem_word_struct(s: String) -> StemWord {
    StemWord {
        word: s.clone(),
        stem: stem_word(s),
    }
}

// pub fn filter_lang(v: Vec<String>, lang: Lang) -> Vec<String> {
//     return v
//         .into_par_iter()
//         .filter(|q| {
//             let info = detect(q).unwrap();
//             info.lang() == lang && info.is_reliable()
//         })
//         .collect();
// }
// pub fn prepare_words(v: Vec<String>, lang: Lang) -> Vec<String> {
//     return v
//         .into_par_iter()
//         .filter(|q| {
//             let info = detect(q).unwrap();
//             info.lang() == lang && info.is_reliable()
//         })
//         .map(|s| stem_word(s))
//         .collect();
// }
