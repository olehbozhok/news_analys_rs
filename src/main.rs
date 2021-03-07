extern crate news_analys_rs;
use news_analys_rs::*;

use whatlang::Lang;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

// use stemmer::Stemmer;

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

    let sentences = get_data("newsAgregatorDB.json").unwrap();
    println!("raw data count {}", sentences.len());

    let stem_sentence = prepare_sentences(sentences, Lang::Ukr);
    println!("filtered by len data count {}", stem_sentence.len());

    let split_at = 1000;
    let (stem_sentence, _) = stem_sentence.split_at(split_at);

    let mut sentences_and_words = vec![];
    stem_sentence.iter().for_each(|sentence| {
        let mut result = Vec::with_capacity(sentence.stem.len());

        sentence
            .stem
            .iter()
            .for_each(|word| result.push(word.stem.as_str()));

        sentences_and_words.push(result);
    });

    println!("start build similarity_matrix");
    let symilarity_matrix = summarizer::build_similarity_matrix(&sentences_and_words, &stop_words);
    println!("done build similarity_matrix");

    let mut symilarity_vals = vec![];
    symilarity_matrix.iter().for_each(|&v| {
        if v >= f64::EPSILON {
            symilarity_vals.push(v)
        }
    });

    let mut out_writer = Box::new(BufWriter::new(File::create(&Path::new("out.txt")).unwrap()));
    out_writer.write(b"Test output\n").unwrap();

    println!("start calc mediana");
    let symilarity_median = calculate_mediana(&mut symilarity_vals);
    println!("done calc mediana");

    writeln!(&mut out_writer, " median {}", symilarity_median).unwrap();

    let sh = symilarity_matrix.shape();
    let (i_max, j_max) = (sh[0] - 1, sh[1] - 1);

    for i in 0..i_max {
        let mut writed = false;

        let j_min = if i < 200 { 0 } else { i - 200 };
        for j in j_min..i {
            let sym = symilarity_matrix[[i, j]];
            if sym > symilarity_median + symilarity_median * 1.0 / 51.0 {
                if !writed {
                    writeln!(&mut out_writer, "!!!!! {} {}", i, stem_sentence[i].origin).unwrap();
                    writed = true;
                }
                writeln!(&mut out_writer, "   {} {}", sym, stem_sentence[j].origin).unwrap();
            } else {
                // writeln!(&mut out_writer, "i:{} j:{} - {}", i, j, sym);
            }
        }
    }
    println!("done")
}

fn calculate_mediana(vector: &mut Vec<f64>) -> f64 {
    vector.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let middle_index = vector.len() / 2;
    *vector.get(middle_index).unwrap()
}
