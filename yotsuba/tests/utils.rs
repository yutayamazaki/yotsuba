#[cfg(test)]
mod tests {
    use yotsuba::normalize;
    use yotsuba::pad_sequence;
    use yotsuba::pad_sequences;
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
    fn pad_sequence_works() {
        let sequence = vec![1, 2, 3];
        assert_eq!(pad_sequence(&sequence, 4, 0), vec![1, 2, 3, 0]);
    }

    #[test]
    fn pad_sequences_works() {
        let sequences = vec![vec![1, 2, 3], vec![0, 2]];
        assert_eq!(pad_sequences(&sequences, 4, 0), vec![vec![1, 2, 3, 0], vec![0, 2, 0, 0]]);
    }
}
