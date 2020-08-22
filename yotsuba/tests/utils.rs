#[cfg(test)]
mod tests {
    use yotsuba::ja::normalize;
    use yotsuba::utils::get_stopwords;
    use yotsuba::utils::get_stopwords_by_frequency;
    use yotsuba::utils::pad_sequence;
    use yotsuba::utils::pad_sequence_post;
    use yotsuba::utils::pad_sequence_pre;
    use yotsuba::utils::pad_sequences;
    use yotsuba::utils::remove_stopwords;
    #[test]
    fn normalize_works() {
        // Test cases from: https://github.com/neologd/mecab-ipadic-neologd/wiki/Regexp.ja#python-written-by-hideaki-t--overlast
        let full_digits = "０１２３４５６７８９";
        assert_eq!(normalize(full_digits), "0123456789");
        println!("{} -> {}", full_digits, normalize(full_digits));

        let full_large_alphabets = "ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺ";
        assert_eq!(
            normalize(full_large_alphabets),
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        );
        println!(
            "{} -> {}",
            full_large_alphabets,
            normalize(full_large_alphabets)
        );

        let full_alphabets = "ａｂｃｄｅｆｇｈｉｊｋｌｍｎｏｐｑｒｓｔｕｖｗｘｙｚ";
        assert_eq!(normalize(full_alphabets), "abcdefghijklmnopqrstuvwxyz");
        println!("{} -> {}", full_alphabets, normalize(full_alphabets));

        // let full_symbols = "！”＃＄％＆’（）＊＋，－．／：；＜＞？＠［￥］＾＿｀｛｜｝";
        // assert_eq!(normalize(full_symbols), "!\"#$%&'()*+,-./:;<>?@[¥]^_`{|}");
        // println!("{} -> {}", full_symbols, normalize(full_symbols));

        let problem2 = "=｡､･｢｣";
        assert_eq!(normalize(problem2), "＝。、・「」");
        println!("{} -> {}", problem2, normalize(problem2));

        // assert "ハンカク" == normalize_neologd("ﾊﾝｶｸ")
        let problem3 = "ﾊﾝｶｸ";
        assert_eq!(normalize(problem3), "ハンカク");
        println!("{} -> {}", problem3, normalize(problem3));

        // assert "o-o" == normalize_neologd("o₋o")
        let problem4 = "o₋o";
        assert_eq!(normalize(problem4), "o-o");
        println!("{} -> {}", problem4, normalize(problem4));

        // assert "majikaー" == normalize_neologd("majika━")
        let mut problem = "majika━";
        assert_eq!(normalize(problem), "majikaー");
        println!("{} -> {}", problem, normalize(problem));

        // assert "わい" == normalize_neologd("わ〰い")
        problem = "わ〰い";
        assert_eq!(normalize(problem), "わい");
        println!("{} -> {}", problem, normalize(problem));

        // assert "スーパー" == normalize_neologd("スーパーーーー")
        problem = "スーパーーーー";
        assert_eq!(normalize(problem), "スーパー");
        println!("{} -> {}", problem, normalize(problem));

        // assert "!#" == normalize_neologd("!#")
        problem = "!#";
        assert_eq!(normalize(problem), "!#");
        println!("{} -> {}", problem, normalize(problem));

        // assert "ゼンカクスペース" == normalize_neologd("ゼンカク　スペース")
        // 多分テストケースがおかしい
        problem = "ゼンカクスペース";
        assert_eq!(normalize(problem), "ゼンカクスペース");
        println!("{} -> {}", problem, normalize(problem));

        // assert "おお" == normalize_neologd("お             お")
        problem = "お             お";
        assert_eq!(normalize(problem), "おお");
        println!("{} -> {}", problem, normalize(problem));

        // assert "おお" == normalize_neologd("      おお")
        problem = "      おお";
        assert_eq!(normalize(problem), "おお");
        println!("{} -> {}", problem, normalize(problem));

        // assert "おお" == normalize_neologd("おお      ")
        problem = "おお      ";
        assert_eq!(normalize(problem), "おお");
        println!("{} -> {}", problem, normalize(problem));

        // assert "検索エンジン自作入門を買いました!!!" == normalize_neologd("検索 エンジン 自作 入門 を 買い ました!!!")
        problem = "検索 エンジン 自作 入門 を 買い ました!!!";
        assert_eq!(normalize(problem), "検索エンジン自作入門を買いました!!!");
        println!("{} -> {}", problem, normalize(problem));

        // assert "アルゴリズムC" == normalize_neologd("アルゴリズム C")
        problem = "アルゴリズム C";
        assert_eq!(normalize(problem), "アルゴリズムC");
        println!("{} -> {}", problem, normalize(problem));

        // assert "PRML副読本" == normalize_neologd("　　　ＰＲＭＬ　　副　読　本　　　")
        problem = "　　　ＰＲＭＬ　　副　読　本　　　";
        println!("{} -> {}", problem, normalize(problem));
        assert_eq!(normalize(problem), "PRML副読本");

        // assert "Coding the Matrix" == normalize_neologd("Coding the Matrix")
        problem = "Coding the Matrix";
        assert_eq!(normalize(problem), "Coding the Matrix");
        println!("{} -> {}", problem, normalize(problem));

        // assert "南アルプスの天然水Sparking Lemonレモン一絞り" == normalize_neologd("南アルプスの　天然水　Ｓｐａｒｋｉｎｇ　Ｌｅｍｏｎ　レモン一絞り")
        problem = "南アルプスの　天然水　Ｓｐａｒｋｉｎｇ　Ｌｅｍｏｎ　レモン一絞り";
        assert_eq!(
            normalize(problem),
            "南アルプスの天然水Sparking Lemonレモン一絞り"
        );
        println!("{} -> {}", problem, normalize(problem));
    }

    #[test]
    fn pad_sequences_works() {
        // Argument maxlen
        let sequences = vec![vec![1, 2, 3], vec![0, 2]];

        assert_eq!(
            pad_sequences(&sequences, Some(3), None, None).unwrap(),
            vec![vec![1, 2, 3], vec![0, 2, 0]]
        );
        assert_eq!(
            pad_sequences(&sequences, Some(4), None, None).unwrap(),
            vec![vec![1, 2, 3, 0], vec![0, 2, 0, 0]]
        );
        assert_eq!(
            pad_sequences(&sequences, None, None, None).unwrap(),
            pad_sequences(&sequences, Some(3), None, None).unwrap()
        );

        // Argument value
        assert_eq!(
            pad_sequences(&sequences, None, None, None).unwrap(),
            vec![vec![1, 2, 3], vec![0, 2, 0]]
        );
        assert_eq!(
            pad_sequences(&sequences, None, Some(1), None).unwrap(),
            vec![vec![1, 2, 3], vec![0, 2, 1]]
        );
        assert_eq!(
            pad_sequences(&sequences, None, Some(0), None).unwrap(),
            pad_sequences(&sequences, None, None, None).unwrap()
        );

        // Argument padding
        assert_eq!(
            pad_sequences(&sequences, None, None, None).unwrap(),
            pad_sequences(&sequences, None, None, Some("post")).unwrap()
        );
        assert_eq!(
            pad_sequences(&sequences, None, None, Some("pre")).unwrap(),
            vec![vec![1, 2, 3], vec![0, 0, 2]]
        );
    }

    #[test]
    fn pad_sequence_pre_works() {
        let sequence = vec![1, 2, 3];
        assert_eq!(pad_sequence_pre(&sequence, 5, None), vec![0, 0, 1, 2, 3]);
    }

    #[test]
    fn pad_sequence_pre_maxlen_smaller_than_seqlen() {
        let sequence = vec![1, 2, 3];
        assert_eq!(pad_sequence_pre(&sequence, 2, None), vec![1, 2]);
    }

    #[test]
    fn pad_sequence_post_works() {
        let sequence = vec![1, 2, 3];
        assert_eq!(
            pad_sequence_post(&sequence, 5, Some(5)),
            vec![1, 2, 3, 5, 5]
        );
    }

    #[test]
    fn pad_sequence_post_maxlen_smaller_than_seqlen() {
        let sequence = vec![1, 2, 3];
        assert_eq!(pad_sequence_post(&sequence, 2, None), vec![1, 2]);
    }

    #[test]
    fn pad_sequence_pre_and_post() {
        let sequence = vec![1, 2, 3];
        assert_eq!(
            pad_sequence(&sequence, 5, None, Some("pre")).unwrap(),
            vec![0, 0, 1, 2, 3]
        );
        assert_eq!(
            pad_sequence(&sequence, 5, None, Some("post")).unwrap(),
            vec![1, 2, 3, 0, 0]
        );
    }

    #[test]
    fn remove_stopwords_works() {
        let tokens = vec!["I", "am", "a", "dog"];
        let stopwords = vec!["am", "a"];
        let ret = remove_stopwords(&tokens, &stopwords);
        assert_eq!(ret, vec!["I", "dog"]);
    }

    #[test]
    fn get_stopwords_by_frequency_works() {
        let docs = vec![vec!["I", "am", "a", "dog"], vec!["I", "have", "a", "pen"]];
        let mut ret = get_stopwords_by_frequency(&docs, 2);
        ret.sort();
        assert_eq!(ret, vec!["I", "a"]);
    }

    #[test]
    fn get_stopwords_works() {
        let ret = get_stopwords("ja");
        assert_eq!(ret.unwrap().len(), 310);
    }
}