use std::collections::HashMap;

use regex::Regex;

use crate::consts;

#[derive(Debug)]
pub enum YotsubaError {
    /// We support only "pre" and "post" as padding argument.
    PaddingDoesNotSupport,
    LangDoesNotSupport,
}

impl std::fmt::Display for YotsubaError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        use YotsubaError::*;
        match self {
            PaddingDoesNotSupport => write!(fmt, "Argument 'padding' must be 'pre' or 'post'."),
            LangDoesNotSupport => write!(fmt, "argment lang must be ja."),
        }
    }
}
impl std::error::Error for YotsubaError {}

pub fn pad_sequence_post(sequence: &Vec<i32>, maxlen: usize, value: Option<i32>) -> Vec<i32> {
    // sequence=[0, 1, 2], maxlen=5, value=10 -> [0, 1, 2, 10, 10]
    let mut ret: Vec<i32> = Vec::new();
    let pad_value = value.unwrap_or(0);
    let seq_len = sequence.len();
    for i in 0..maxlen {
        let x: i32;
        let idx = i as usize;
        if seq_len > idx {
            x = sequence[idx];
        } else {
            x = pad_value;
        }
        ret.push(x);
    }
    ret
}

pub fn pad_sequence_pre(sequence: &Vec<i32>, maxlen: usize, value: Option<i32>) -> Vec<i32> {
    // sequence=[0, 1, 2], maxlen=5, value=10 -> [10, 10, 0, 1, 2]
    let mut ret: Vec<i32> = Vec::new();
    // If maxlen is smaller than given sequence length.
    if maxlen < sequence.len() {
        for i in 0..maxlen {
            let idx = i as usize;
            ret.push(sequence[idx]);
        }
        return ret;
    }
    let pad_size = maxlen - sequence.len();
    let pad_value = value.unwrap_or(0);
    for i in 0..maxlen {
        let x: i32;
        let idx = i as usize;
        if idx < pad_size {
            x = pad_value;
        } else {
            x = sequence[idx - pad_size];
        }
        ret.push(x);
    }
    ret
}

pub fn pad_sequence(
    sequence: &Vec<i32>,
    maxlen: usize,
    value: Option<i32>,
    padding: Option<&str>,
) -> Result<Vec<i32>, YotsubaError> {
    // sequence=[0, 1, 2], maxlen=5, value=10 -> [0, 1, 2, 10, 10]
    let padding_ = padding.unwrap_or("post");
    match padding_ {
        "post" => return Ok(pad_sequence_post(sequence, maxlen, value)),
        "pre" => return Ok(pad_sequence_pre(sequence, maxlen, value)),
        __ => return Err(YotsubaError::PaddingDoesNotSupport),
    };
}

pub fn pad_sequences(
    sequences: &Vec<Vec<i32>>,
    maxlen: Option<usize>,
    value: Option<i32>,
    padding: Option<&str>,
) -> Result<Vec<Vec<i32>>, YotsubaError> {
    // sequences=[[0, 1, 2], ...], maxlen=5, value=10 -> [[0, 1, 2, 10, 10], ...]
    let mut padding_ = padding.unwrap_or("post");
    padding_ = match padding_ {
        "post" => "post",
        "pre" => "pre",
        _ => return Err(YotsubaError::PaddingDoesNotSupport),
    };

    let mut seq_maxlen = 0;
    if maxlen.is_none() {
        for sequence in sequences {
            let seq_len = sequence.len();
            if seq_len > seq_maxlen {
                seq_maxlen = seq_len;
            }
        }
    }
    seq_maxlen = maxlen.unwrap_or(seq_maxlen);
    let pad_value = value.unwrap_or(0);

    let mut ret: Vec<Vec<i32>> = Vec::new();
    for sequence in sequences {
        let pad = pad_sequence(sequence, seq_maxlen, Some(pad_value), Some(padding_));
        match pad {
            Ok(v) => ret.push(v),
            Err(_) => return Err(YotsubaError::PaddingDoesNotSupport),
        };
    }
    Ok(ret)
}

pub fn remove_stopwords(tokens: &Vec<&str>, stopwords: &Vec<&str>) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    for i in 0..tokens.len() {
        let token = tokens[i];
        if !stopwords.contains(&token) {
            ret.push(token.to_string());
        }
    }
    ret
}

pub fn clean_url(text: &str, replace: Option<&str>) -> String {
    let rep = Regex::new(r"http\S+").unwrap();
    let replace_ = replace.unwrap_or("");
    rep.replace_all(text, replace_).to_string()
}

pub fn clean_html_tags(text: &str, replace: Option<&str>) -> String {
    let rep = Regex::new(r"<.+?>").unwrap();
    let replace_ = replace.unwrap_or("");
    rep.replace_all(text, replace_).to_string()
}

pub fn clean_emails(text: &str, replace: Option<&str>) -> String {
    let rep = Regex::new(r"[\w\-._]+@[\w\-._]+\.[A-Za-z]+").unwrap();
    let replace_ = replace.unwrap_or("");
    rep.replace_all(text, replace_).to_string()
}

pub fn clean_number(text: &str, replace: Option<&str>) -> String {
    let rep = Regex::new(r"\d+").unwrap();
    let replace_ = replace.unwrap_or("0");
    rep.replace_all(text, replace_).to_string()
}

pub fn clean_emoji(text: &str, replace: Option<&str>) -> String {
    // Maybe it's better?? https://gist.github.com/slowkow/7a7f61f495e3dbb7e3d767f97bd7304b
    // regular expressions from https://github.com/rust-lang/regex/issues/645
    let rep = Regex::new(r"\p{Emoji}").unwrap();
    let replace_ = replace.unwrap_or("");
    rep.replace_all(text, replace_).to_string()
}

pub fn count_token_frequency(docs: &Vec<Vec<&str>>) -> HashMap<String, u32> {
    // Count frequency of given tokens.
    let mut frequency: HashMap<String, u32> = HashMap::new();
    for doc in docs {
        for token in doc {
            *frequency.entry(token.to_string()).or_insert(0) += 1;
        }
    }
    frequency
}

pub fn get_stopwords_by_frequency(
    docs: &Vec<Vec<&str>>,
    max_freq: u32,
    // min_freq: u32
) -> Vec<String> {
    let frequency: HashMap<String, u32> = count_token_frequency(docs);
    frequency
        .iter()
        .filter(|&(_, freq)| freq >= &max_freq)
        .map(|(token, _)| token.clone())
        .collect()
}

pub fn get_stopwords(lang: &str) -> Result<Vec<&str>, YotsubaError> {
    match lang {
        "ja" => return Ok(consts::STOPWORDS_JA.to_vec()),
        _ => return Err(YotsubaError::LangDoesNotSupport),
    };
}
