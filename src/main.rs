extern crate news_analys_rs;
use news_analys_rs::*;

use whatlang::Lang;

// use stemmer::Stemmer;

fn main() {
    println!("start");

    let stop_words: Vec<String> = load_stop_words("stop_words.txt")
        .unwrap()
        .iter()
        .map(|s| stem_word(s.clone()))
        .collect();
    println!("{:#?}", stop_words);

    let sentences = get_data("newsAgregatorDB.json").unwrap();
    println!("raw data count {}", sentences.len());

    let v = prepare_sentences(sentences, Lang::Ukr);
    println!("filtered by len data count {}", v.len());

    // let morph_uk = MorphAnalyzer::from_file(rsmorphy_dict_uk::DICT_PATH);
    // let morph_res = morph_uk.parse("ходив");
    // morph_res
    //     .into_iter()
    //     .for_each(|v| println!("{}", v.lex.get_word()));

    // for language in Stemmer::list() {
    //     println!("{}", language);
    // }

    // let mut stemmer = Stemmer::new("english").unwrap();
    // let stemmed: &str = stemmer.stem_str("foo");
}
