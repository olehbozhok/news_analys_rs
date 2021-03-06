use std::path::Path;
use std::{fs, io::Read};

use serde_json;
use serde_json::Value;

type VecStr = Vec<String>;

fn get_titles(v: &Value, titles: &mut VecStr) {
    if let Some(v) = v.get("Title") {
        titles.push(v.to_string());
    };
    if let Some(v) = v.get("News") {
        if let Some(v) = v.as_array() {
            titles.reserve(v.len());
            v.iter().for_each(|q| get_titles(q, titles));
        }
    }
}

pub fn get_data<P: AsRef<Path>>(path: P) -> Option<VecStr> {
    let mut buf = String::new();
    {
        let _file = fs::File::open(path)
            .expect("could not open file")
            .read_to_string(&mut buf)
            .unwrap();
    }
    let json: serde_json::Value =
        serde_json::from_str(buf.as_str()).expect("file should be proper JSON");

    let json_arr = json.as_array().unwrap();

    let mut titles: VecStr = Vec::new();

    json_arr.iter().for_each(|v| get_titles(v, &mut titles));

    return Some(titles);
}

pub fn load_stop_words<P: AsRef<Path>>(path: P) -> Option<VecStr> {
    let mut buf = String::new();
    {
        let _file = fs::File::open(path)
            .expect("could not open file")
            .read_to_string(&mut buf)
            .unwrap();
    }

    let spl = buf
        .split("\n")
        .filter_map(|s| {
            if s.len() == 0 {
                None
            } else {
                Some(s.replace("\r", "").into())
            }
        })
        .collect();
    Some(spl)
}
