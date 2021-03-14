extern crate news_analys_rs;
use news_analys_rs::*;

use whatlang::Lang;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use ngrams::Ngrams;

const DAY_IN_UNIX_TIME: i64 = 60 * 60 * 60;

fn main() {
    println!("start");

    let stop_words_string: Vec<String> = load_stop_words("stop_words.txt")
        .unwrap()
        .iter()
        .map(|s| stem_word(s.clone()))
        .collect();
    // println!("{:#?}", stop_words);

    let mut stop_words = vec![];
    stop_words_string
        .iter()
        .for_each(|s| stop_words.push(s.as_str()));

    let sentences_data = get_data("newsAgregatorDB.json").unwrap();
    println!("raw data count {}", sentences_data.len());

    let sentences: Vec<&str> = sentences_data
        .iter()
        .map(|data| data.title.as_str())
        .collect();

    let stem_sentence = prepare_sentences(sentences, Lang::Ukr);
    println!("filtered by len data count {}", stem_sentence.len());

    // let split_at = 1000;
    let split_at = stem_sentence.len();

    let (stem_sentence, _) = stem_sentence.split_at(split_at);

    let mut min_date = sentences_data[0].created;
    let mut max_date = sentences_data[0].created;
    sentences_data.iter().for_each(|s| {
        if min_date > s.created {
            min_date = s.created
        }
        if max_date < s.created {
            max_date = s.created
        }
    });

    let mut iter_n = 0;
    while min_date < max_date {
        min_date += DAY_IN_UNIX_TIME;

        let stem_sentence_to_handle: Vec<&StemSentence> = stem_sentence
            .iter()
            .filter(|v| {
                (min_date - DAY_IN_UNIX_TIME) < sentences_data[v.origin_id].created
                    && min_date > sentences_data[v.origin_id].created
            })
            .map(|v| v)
            .collect();

        println!("to handle {}", stem_sentence_to_handle.len());
        if stem_sentence_to_handle.len() == 0 {
            continue;
        }

        do_work(
            iter_n,
            sentences_data.as_slice(),
            stem_sentence_to_handle.as_slice(),
            stop_words.as_slice(),
        );
        iter_n += 1;
    }
}

fn do_work(
    iter_n: i64,
    sentences_data: &[Data],
    stem_sentence: &[&StemSentence],
    stop_words: &[&str],
) {
    let (mut first_created, mut last_created) =
        (sentences_data[0].created, sentences_data[0].created);
    &stem_sentence.iter().for_each(|v| {
        let created = sentences_data[v.origin_id].created;
        if created > last_created {
            last_created = created;
        }
        if created < first_created {
            first_created = created;
        }
    });
    let mut sentences_and_words = vec![];
    let mut sentences_and_ngrams = vec![];
    stem_sentence.iter().for_each(|sentence| {
        let mut result = Vec::with_capacity(sentence.stem.len());

        sentence
            .stem
            .iter()
            .for_each(|word| result.push(word.stem.as_str()));
        sentences_and_words.push(result.clone());

        let iter = result.into_iter();
        let grams: Vec<_> = Ngrams::new(iter, 3).pad().map(|v| v.join(" ")).collect();
        sentences_and_ngrams.push(grams);
    });
    let mut slice_sentences_and_ngrams: Vec<Vec<&str>> = sentences_and_ngrams
        .iter()
        .map(|v| v.iter().map(|v2| v2.as_str()).collect())
        .collect();
    sentences_and_words
        .iter_mut()
        .enumerate()
        .for_each(|(i, v)| v.append(&mut slice_sentences_and_ngrams[i]));

    println!("start build similarity_matrix");
    let symilarity_matrix = summarizer::build_similarity_matrix(&sentences_and_words, &stop_words);
    println!("done build similarity_matrix");

    let mut out_writer = Box::new(BufWriter::new(
        File::create(&Path::new(format!("out_{}.txt", iter_n).as_str())).unwrap(),
    ));
    out_writer.write(b"Test output\n").unwrap();
    let (i_max, _j_max) = symilarity_matrix.shape();
    for i in 0..i_max {
        let mut writed = false;

        let j_min = if i < 200 { 0 } else { i - 200 };
        for j in j_min..i {
            let sym = match symilarity_matrix.get(i, j) {
                Some(v) => *v,
                None => 0.0,
            };

            if sym >= 0.1 {
                if !writed {
                    writeln!(
                        &mut out_writer,
                        "!!!!! {} {}",
                        i, &sentences_data[stem_sentence[i].origin_id].title
                    )
                    .expect("err write to file");
                    writed = true;
                }
                writeln!(
                    &mut out_writer,
                    "   {} {}",
                    sym, sentences_data[stem_sentence[j].origin_id].title
                )
                .unwrap();
            } else {
                // writeln!(&mut out_writer, "i:{} j:{} - {}", i, j, sym);
            }
        }
    }
    println!("done");
}
