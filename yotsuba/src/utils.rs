use std::collections::HashMap;

const DIGITS: [(char, char); 10] = [
    ('０', '0'),
    ('１', '1'),
    ('２', '2'),
    ('３', '3'),
    ('４', '4'),
    ('５', '5'),
    ('６', '6'),
    ('７', '7'),
    ('８', '8'),
    ('９', '9'),
];
const ASCII: [(char, char); 84] = [
    ('ａ', 'a'),
    ('ｂ', 'b'),
    ('ｃ', 'c'),
    ('ｄ', 'd'),
    ('ｅ', 'e'),
    ('ｆ', 'f'),
    ('ｇ', 'g'),
    ('ｈ', 'h'),
    ('ｉ', 'i'),
    ('ｊ', 'j'),
    ('ｋ', 'k'),
    ('ｌ', 'l'),
    ('ｍ', 'm'),
    ('ｎ', 'n'),
    ('ｏ', 'o'),
    ('ｐ', 'p'),
    ('ｑ', 'q'),
    ('ｒ', 'r'),
    ('ｓ', 's'),
    ('ｔ', 't'),
    ('ｕ', 'u'),
    ('ｖ', 'v'),
    ('ｗ', 'w'),
    ('ｘ', 'x'),
    ('ｙ', 'y'),
    ('ｚ', 'z'),
    ('Ａ', 'A'),
    ('Ｂ', 'B'),
    ('Ｃ', 'C'),
    ('Ｄ', 'D'),
    ('Ｅ', 'E'),
    ('Ｆ', 'F'),
    ('Ｇ', 'G'),
    ('Ｈ', 'H'),
    ('Ｉ', 'I'),
    ('Ｊ', 'J'),
    ('Ｋ', 'K'),
    ('Ｌ', 'L'),
    ('Ｍ', 'M'),
    ('Ｎ', 'N'),
    ('Ｏ', 'O'),
    ('Ｐ', 'P'),
    ('Ｑ', 'Q'),
    ('Ｒ', 'R'),
    ('Ｓ', 'S'),
    ('Ｔ', 'T'),
    ('Ｕ', 'U'),
    ('Ｖ', 'V'),
    ('Ｗ', 'W'),
    ('Ｘ', 'X'),
    ('Ｙ', 'Y'),
    ('Ｚ', 'Z'),
    ('！', '!'),
    ('”', '"'),
    ('＃', '#'),
    ('＄', '$'),
    ('％', '%'),
    ('＆', '&'),
    ('’', '\''),
    ('（', '('),
    ('）', ')'),
    ('＊', '*'),
    ('＋', '+'),
    ('，', ','),
    ('−', '-'),
    ('．', '.'),
    ('／', '/'),
    ('：', ':'),
    ('；', ';'),
    ('＜', '<'),
    ('=', '＝'),
    ('＞', '>'),
    ('？', '?'),
    ('＠', '@'),
    ('［', '['),
    ('￥', '¥'),
    ('｀', '`'),
    ('］', ']'),
    ('＾', '^'),
    ('＿', '_'),
    ('‘', '`'),
    ('｛', '{'),
    ('｜', '|'),
    ('｝', '}'),
];
const KANA: [(char, char); 63] = [
    ('ｱ', 'ア'),
    ('ｲ', 'イ'),
    ('ｳ', 'ウ'),
    ('ｴ', 'エ'),
    ('ｵ', 'オ'),
    ('ｶ', 'カ'),
    ('ｷ', 'キ'),
    ('ｸ', 'ク'),
    ('ｹ', 'ケ'),
    ('ｺ', 'コ'),
    ('ｻ', 'サ'),
    ('ｼ', 'シ'),
    ('ｽ', 'ス'),
    ('ｾ', 'セ'),
    ('ｿ', 'ソ'),
    ('ﾀ', 'タ'),
    ('ﾁ', 'チ'),
    ('ﾂ', 'ツ'),
    ('ﾃ', 'テ'),
    ('ﾄ', 'ト'),
    ('ﾅ', 'ナ'),
    ('ﾆ', 'ニ'),
    ('ﾇ', 'ヌ'),
    ('ﾈ', 'ネ'),
    ('ﾉ', 'ノ'),
    ('ﾊ', 'ハ'),
    ('ﾋ', 'ヒ'),
    ('ﾌ', 'フ'),
    ('ﾍ', 'ヘ'),
    ('ﾎ', 'ホ'),
    ('ﾏ', 'マ'),
    ('ﾐ', 'ミ'),
    ('ﾑ', 'ム'),
    ('ﾒ', 'メ'),
    ('ﾓ', 'モ'),
    ('ﾔ', 'ヤ'),
    ('ﾕ', 'ユ'),
    ('ﾖ', 'ヨ'),
    ('ﾗ', 'ラ'),
    ('ﾘ', 'リ'),
    ('ﾙ', 'ル'),
    ('ﾚ', 'レ'),
    ('ﾛ', 'ロ'),
    ('ﾜ', 'ワ'),
    ('ｦ', 'ヲ'),
    ('ﾝ', 'ン'),
    ('ｧ', 'ァ'),
    ('ｨ', 'ィ'),
    ('ｩ', 'ゥ'),
    ('ｪ', 'ェ'),
    ('ｫ', 'ォ'),
    ('ｯ', 'ッ'),
    ('ｬ', 'ャ'),
    ('ｭ', 'ュ'),
    ('ｮ', 'ョ'),
    ('｡', '。'),
    ('､', '、'),
    ('･', '・'),
    ('゛', 'ﾞ'),
    ('゜', 'ﾟ'),
    ('｢', '「'),
    ('｣', '」'),
    ('ｰ', 'ー'),
];
const HIPHENS: [char; 10] = ['˗', '֊', '‐', '‑', '‒', '–', '⁃', '⁻', '₋', '−'];
const CHOONPUS: [char; 8] = ['﹣', '－', 'ｰ', '—', '―', '─', '━', 'ー'];
const TILDES: [char; 6] = ['~', '∼', '∾', '〜', '〰', '～'];
const SPACE: [char; 2] = [' ', '　'];

fn is_kanji(c: &char) -> bool {
    (*c >= '\u{4e00}' && *c <= '\u{9ffc}') // Standard set.
        || (*c >= '\u{f900}' && *c <= '\u{faff}') // CJK Compatibility Ideographs.
}

fn is_hiragana(c: &char) -> bool {
    *c >= '\u{3041}' && *c <= '\u{309f}'
}

fn is_katakana(c: &char) -> bool {
    *c >= '\u{30a0}' && *c <= '\u{30ff}'
}

fn is_japanese_punct(c: &char) -> bool {
    *c >= '\u{3000}' && *c <= '\u{303f}'
}

fn is_alphanum(c: &char) -> bool {
    *c >= '\u{ff01}' && *c <= '\u{ff5e}'
}

fn is_in_blocks(c: &char) -> bool {
    is_kanji(c) || is_hiragana(c) || is_katakana(c) || is_japanese_punct(c) || is_alphanum(c)
}

pub fn normalize(text: &str) -> String {
    // Create hashmap to convert chars
    let mut map = HashMap::new();
    for (before, after) in DIGITS.iter() {
        map.insert(before, after);
    }
    for (before, after) in ASCII.iter() {
        map.insert(before, after);
    }
    for (before, after) in KANA.iter() {
        map.insert(before, after);
    }

    let mut ret = "".to_string();
    let mut prev = ' ';
    let mut c: char;
    for s in text.chars() {
        c = s;
        if map.contains_key(&s) {
            c = *map[&s];
            ret.push(c);
        } else if HIPHENS.contains(&s) {
            if prev == '-' {
                continue;
            }
            c = '-';
            ret.push(c);
        } else if CHOONPUS.contains(&c) {
            if prev == 'ー' {
                continue;
            }
            c = 'ー';
            prev = c;
            ret.push(c);
            continue;
        } else if TILDES.contains(&c) {
            prev = c;
            continue;
        } else if SPACE.contains(&c) {
            // 全角スペースは半角スペースに
            c = ' ';
            if prev == ' ' || is_in_blocks(&prev) {
                continue;
            }
            if prev.is_ascii() {
                ret.push(c);
                prev = c;
                continue;
            } else if SPACE.contains(&prev) {
                prev = c;
                continue;
            }
        } else {
            if prev == ' ' && is_in_blocks(&c) {
                ret.pop();
            }
            ret.push(c);
        }
        prev = c;
    }
    ret
}

pub fn pad_sequence(sequence: &Vec<i32>, maxlen: i32, value: i32) -> Vec<i32> {
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


pub fn pad_sequences(sequences: &Vec<Vec<i32>>, maxlen: i32, value: i32) -> Vec<Vec<i32>> {
    // sequences=[[0, 1, 2], ...], maxlen=5, value=10 -> [[0, 1, 2, 10, 10], ...]
    let mut ret: Vec<Vec<i32>> = Vec::new();
    for sequence in sequences {
        ret.push(pad_sequence(sequence, maxlen, value));
    }
    ret
}
