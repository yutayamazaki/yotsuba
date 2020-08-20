#[derive(Debug)]
pub enum PadSequenceError {
    /// We support only "pre" and "post" as padding argument.
    PaddingDoesNotSupport,
}

impl std::fmt::Display for PadSequenceError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        use PadSequenceError::*;
        match self {
            PaddingDoesNotSupport => write!(fmt, "Argument 'padding' must be 'pre' or 'post'."),
        }
    }
}
impl std::error::Error for PadSequenceError {}

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
) -> Vec<i32> {
    // sequence=[0, 1, 2], maxlen=5, value=10 -> [0, 1, 2, 10, 10]
    let padding_ = padding.unwrap_or("post");
    if padding_ == "pre" {
        return pad_sequence_pre(sequence, maxlen, value);
    } else {
        return pad_sequence_post(sequence, maxlen, value);
    }
}

pub fn pad_sequences(
    sequences: &Vec<Vec<i32>>,
    maxlen: Option<usize>,
    value: Option<i32>,
    padding: Option<&str>,
) -> Result<Vec<Vec<i32>>, PadSequenceError> {
    // sequences=[[0, 1, 2], ...], maxlen=5, value=10 -> [[0, 1, 2, 10, 10], ...]
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
    let mut padding_ = padding.unwrap_or("post");
    padding_ = match padding_ {
        "post" => "post",
        "pre" => "pre",
        _ => return Err(PadSequenceError::PaddingDoesNotSupport),
    };
    for sequence in sequences {
        ret.push(pad_sequence(
            sequence,
            seq_maxlen,
            Some(pad_value),
            Some(padding_),
        ));
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
