pub fn pad_sequence(sequence: &Vec<i32>, maxlen: usize, value: i32) -> Vec<i32> {
    // sequence=[0, 1, 2], maxlen=5, value=10 -> [0, 1, 2, 10, 10]
    let mut ret: Vec<i32> = Vec::new();
    for i in 0..maxlen {
        let x: i32;
        let idx = i as usize;
        if sequence.len() > idx {
            x = sequence[idx];
        } else {
            x = value;
        }
        ret.push(x);
    }
    ret
}

pub fn pad_sequences(sequences: &Vec<Vec<i32>>, maxlen: Option<usize>, value: i32) -> Vec<Vec<i32>> {
    // sequences=[[0, 1, 2], ...], maxlen=5, value=10 -> [[0, 1, 2, 10, 10], ...]
    let mut ret: Vec<Vec<i32>> = Vec::new();
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

    for sequence in sequences {
        ret.push(pad_sequence(sequence, seq_maxlen, value));
    }
    ret
}
