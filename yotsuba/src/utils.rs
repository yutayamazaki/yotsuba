use std::collections::HashMap;

use regex::Regex;

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

// http://svn.sourceforge.jp/svnroot/slothlib/CSharp/Version1/SlothLib/NLP/Filter/StopWord/word/Japanese.txt
const STOPWORDS_JA: [&str; 310] = [
    "あそこ",
    "あたり",
    "あちら",
    "あっち",
    "あと",
    "あな",
    "あなた",
    "あれ",
    "いくつ",
    "いつ",
    "いま",
    "いや",
    "いろいろ",
    "うち",
    "おおまか",
    "おまえ",
    "おれ",
    "がい",
    "かく",
    "かたち",
    "かやの",
    "から",
    "がら",
    "きた",
    "くせ",
    "ここ",
    "こっち",
    "こと",
    "ごと",
    "こちら",
    "ごっちゃ",
    "これ",
    "これら",
    "ごろ",
    "さまざま",
    "さらい",
    "さん",
    "しかた",
    "しよう",
    "すか",
    "ずつ",
    "すね",
    "すべて",
    "ぜんぶ",
    "そう",
    "そこ",
    "そちら",
    "そっち",
    "そで",
    "それ",
    "それぞれ",
    "それなり",
    "たくさん",
    "たち",
    "たび",
    "ため",
    "だめ",
    "ちゃ",
    "ちゃん",
    "てん",
    "とおり",
    "とき",
    "どこ",
    "どこか",
    "ところ",
    "どちら",
    "どっか",
    "どっち",
    "どれ",
    "なか",
    "なかば",
    "なに",
    "など",
    "なん",
    "はじめ",
    "はず",
    "はるか",
    "ひと",
    "ひとつ",
    "ふく",
    "ぶり",
    "べつ",
    "へん",
    "ぺん",
    "ほう",
    "ほか",
    "まさ",
    "まし",
    "まとも",
    "まま",
    "みたい",
    "みつ",
    "みなさん",
    "みんな",
    "もと",
    "もの",
    "もん",
    "やつ",
    "よう",
    "よそ",
    "わけ",
    "わたし",
    "ハイ",
    "上",
    "中",
    "下",
    "字",
    "年",
    "月",
    "日",
    "時",
    "分",
    "秒",
    "週",
    "火",
    "水",
    "木",
    "金",
    "土",
    "国",
    "都",
    "道",
    "府",
    "県",
    "市",
    "区",
    "町",
    "村",
    "各",
    "第",
    "方",
    "何",
    "的",
    "度",
    "文",
    "者",
    "性",
    "体",
    "人",
    "他",
    "今",
    "部",
    "課",
    "係",
    "外",
    "類",
    "達",
    "気",
    "室",
    "口",
    "誰",
    "用",
    "界",
    "会",
    "首",
    "男",
    "女",
    "別",
    "話",
    "私",
    "屋",
    "店",
    "家",
    "場",
    "等",
    "見",
    "際",
    "観",
    "段",
    "略",
    "例",
    "系",
    "論",
    "形",
    "間",
    "地",
    "員",
    "線",
    "点",
    "書",
    "品",
    "力",
    "法",
    "感",
    "作",
    "元",
    "手",
    "数",
    "彼",
    "彼女",
    "子",
    "内",
    "楽",
    "喜",
    "怒",
    "哀",
    "輪",
    "頃",
    "化",
    "境",
    "俺",
    "奴",
    "高",
    "校",
    "婦",
    "伸",
    "紀",
    "誌",
    "レ",
    "行",
    "列",
    "事",
    "士",
    "台",
    "集",
    "様",
    "所",
    "歴",
    "器",
    "名",
    "情",
    "連",
    "毎",
    "式",
    "簿",
    "回",
    "匹",
    "個",
    "席",
    "束",
    "歳",
    "目",
    "通",
    "面",
    "円",
    "玉",
    "枚",
    "前",
    "後",
    "左",
    "右",
    "次",
    "先",
    "春",
    "夏",
    "秋",
    "冬",
    "一",
    "二",
    "三",
    "四",
    "五",
    "六",
    "七",
    "八",
    "九",
    "十",
    "百",
    "千",
    "万",
    "億",
    "兆",
    "下記",
    "上記",
    "時間",
    "今回",
    "前回",
    "場合",
    "一つ",
    "年生",
    "自分",
    "ヶ所",
    "ヵ所",
    "カ所",
    "箇所",
    "ヶ月",
    "ヵ月",
    "カ月",
    "箇月",
    "名前",
    "本当",
    "確か",
    "時点",
    "全部",
    "関係",
    "近く",
    "方法",
    "我々",
    "違い",
    "多く",
    "扱い",
    "新た",
    "その後",
    "半ば",
    "結局",
    "様々",
    "以前",
    "以後",
    "以降",
    "未満",
    "以上",
    "以下",
    "幾つ",
    "毎日",
    "自体",
    "向こう",
    "何人",
    "手段",
    "同じ",
    "感じ",
];

pub fn get_stopwords(lang: &str) -> Result<Vec<&str>, YotsubaError> {
    match lang {
        "ja" => return Ok(STOPWORDS_JA.to_vec()),
        _ => return Err(YotsubaError::LangDoesNotSupport),
    };
}
