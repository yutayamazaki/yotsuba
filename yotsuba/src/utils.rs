pub fn pad_sequence(sequence: &Vec<i32>, maxlen: usize, value: Option<i32>) -> Vec<i32> {
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

pub fn pad_sequences(sequences: &Vec<Vec<i32>>, maxlen: Option<usize>, value: Option<i32>) -> Vec<Vec<i32>> {
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
    for sequence in sequences {
        ret.push(pad_sequence(sequence, seq_maxlen, Some(pad_value)));
    }
    ret
}
