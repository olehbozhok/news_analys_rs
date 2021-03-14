use rayon::prelude::*;
use sprs::{CsMat, TriMat};
use std::collections::BTreeSet;
use std::sync::Mutex;
use unicode_segmentation::UnicodeSegmentation;

fn get_all_words_lc<'a>(sentence1: &[&'a str], sentence2: &[&'a str]) -> BTreeSet<String> {
    let mut all_words: BTreeSet<String> = BTreeSet::new();

    sentence1.iter().for_each(|w| {
        all_words.insert(w.to_lowercase());
    });

    sentence2.iter().for_each(|w| {
        all_words.insert(w.to_lowercase());
    });
    return all_words;
}

///
/// Retrieve a sentence vector based on the frequency of words that appears in the all_words_lc set.
/// all_words_lc should be a sorted set of lower cased words
/// The size of the resulting vector is the same as the all_words_lc set
/// stop_words are skipped
///
fn get_sentence_vector(
    sentence: &[&str],
    all_words_lc: &BTreeSet<String>,
    stop_words: &[&str],
) -> Vec<usize> {
    let mut vector: Vec<usize> = vec![0; all_words_lc.len()];
    for word in sentence {
        let word_lc = word.to_lowercase();
        if !stop_words.contains(&word_lc.as_str()) {
            let index = all_words_lc.iter().position(|x| x.eq(&word_lc)).unwrap();
            vector[index] += 1;
        }
    }
    return vector;
}

///
/// Calculates the cosine distance between two vectors
/// Refer to [YouTube](https://www.youtube.com/watch?v=3X0wLRwU_Ws)
///
fn cosine_distance(vec1: &Vec<usize>, vec2: &Vec<usize>) -> f64 {
    let dot_product = dot_product(vec1, vec2);
    let root_sum_square1 = root_sum_square(vec1);
    let root_sum_square2 = root_sum_square(vec2);
    return dot_product as f64 / (root_sum_square1 * root_sum_square2);
}

fn root_sum_square(vec: &Vec<usize>) -> f64 {
    let mut sum_square = 0;
    for i in 0..vec.len() {
        sum_square += vec[i] * vec[i];
    }
    (sum_square as f64).sqrt()
}

fn dot_product(vec1: &Vec<usize>, vec2: &Vec<usize>) -> usize {
    let delta = vec1.len() - vec2.len();
    let shortest_vec = match delta {
        d if d <= 0 => vec1,
        d if d > 0 => vec2,
        _ => vec1,
    };
    let mut dot_product = 0;
    for i in 0..shortest_vec.len() {
        dot_product += vec1[i] * vec2[i];
    }
    dot_product
}

fn sentence_similarity(s1: &[&str], s2: &[&str], stop_words: &[&str]) -> f64 {
    let all_words = get_all_words_lc(s1, s2);
    let v1 = get_sentence_vector(s1, &all_words, stop_words);
    let v2 = get_sentence_vector(s2, &all_words, stop_words);
    cosine_distance(&v1, &v2)
}

///
/// Calculate a similarity matrix for the given sentences.
/// Returns a 2-D array M_i,j such that for all 'j', sum(i, M_i,j) = 1
/// We take a leap of faith here and assume that cosine similarity is similar to the probability
/// that a sentence is important for summarization
///
pub fn build_similarity_matrix(sentences: &Vec<Vec<&str>>, stop_words: &[&str]) -> CsMat<f64> {
    let len = sentences.len();

    let mut matrix_tri = TriMat::new((len, len));
    let matrix_tri_mutex = Mutex::new(&mut matrix_tri);
    let count = Mutex::new(0);
    (0..len).into_par_iter().for_each(|i| {
        for j in 0..len {
            if i == j {
                continue;
            }
            let res =
                sentence_similarity(sentences[i].as_slice(), sentences[j].as_slice(), stop_words);

            if res > f64::EPSILON {
                {
                    let mut matr = matrix_tri_mutex.lock().unwrap();
                    (*matr).add_triplet(i, j, res);
                }
            }
        }
        let c;
        {
            let mut count2 = count.lock().unwrap();
            *count2 += 1;
            c = *count2
        }
        println!("build_similarity_matrix {}%", c as f32 / len as f32 * 100.0);
    });
    matrix_tri.to_csr()
}

///
/// Calculate a sentence rank similar to a page rank.
/// Please refer to [PageRank](https://en.wikipedia.org/wiki/PageRank) for more details.
///
// pub fn calculate_sentence_rank(similarity_matrix: &Array2<f64>) -> Vec<f64> {
//     let num_sentence = similarity_matrix.shape()[1];
//     let threshold = 0.001;
//     // Initialize a vector with the same value 1/number of sentences. Uniformly distributed across
//     // all sentences. NOTE: perhaps we can make some sentences more important than the rest?
//     let initial_vector: Vec<f64> = vec![1.0 / num_sentence as f64; num_sentence];
//     let mut result = Array1::from(initial_vector);
//     let mut prev_result = result.clone();
//     let damping_factor = 0.85;
//     let initial_m =
//         damping_factor * similarity_matrix + (1.0 - damping_factor) / num_sentence as f64;
//     loop {
//         result = initial_m.dot(&result);
//         let delta = &result - &prev_result;
//         let mut converged = true;
//         for i in 0..delta.len() {
//             if delta[i] > threshold {
//                 converged = false;
//                 break;
//             }
//         }
//         if converged {
//             break;
//         }
//         prev_result = result.clone();
//     }
//     result.into_raw_vec()
// }

pub fn split_into_words(sentence: &str) -> Vec<&str> {
    let mut result = vec![];
    let words = sentence.unicode_words();
    for word in words {
        result.push(word);
    }
    result
}
