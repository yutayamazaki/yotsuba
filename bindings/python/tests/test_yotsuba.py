import time
import unittest
from typing import List

import yotsuba


class NormalizeTests(unittest.TestCase):

    def test_rust(self):
        start: float = time.time()
        self.assertEqual(
            yotsuba.ja.normalize_neologd("０１２３４５６７８９"),
            "0123456789"
        )
        self.assertEqual(yotsuba.ja.normalize_neologd(
            "ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺ"), "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        )
        self.assertEqual(
            yotsuba.ja.normalize_neologd("ａｂｃｄｅｆｇｈｉｊｋｌｍｎｏｐｑｒｓｔｕｖｗｘｙｚ"),
            "abcdefghijklmnopqrstuvwxyz"
        )
        # self.assertEqual(yotsuba.normalize_neologd(
        #     "！”＃＄％＆’（）＊＋，－．／：；＜＞？＠［￥］＾＿｀｛｜｝"),
        #     "!\"#$%&'()*+,-./:;<>?@[¥]^_`{|}"
        # )
        self.assertEqual(yotsuba.ja.normalize_neologd("＝。、・「」"), "＝。、・「」")
        self.assertEqual(yotsuba.ja.normalize_neologd("ﾊﾝｶｸ"), "ハンカク")
        self.assertEqual(yotsuba.ja.normalize_neologd("o₋o"), "o-o")
        self.assertEqual(yotsuba.ja.normalize_neologd("majika━"), "majikaー")
        self.assertEqual(yotsuba.ja.normalize_neologd("わ〰い"), "わい")
        self.assertEqual(yotsuba.ja.normalize_neologd("スーパーーーー"), "スーパー")
        self.assertEqual(yotsuba.ja.normalize_neologd("!#"), "!#")
        self.assertEqual(yotsuba.ja.normalize_neologd("ゼンカク　スペース"), "ゼンカクスペース")
        self.assertEqual(yotsuba.ja.normalize_neologd("お             お"), "おお")
        self.assertEqual(yotsuba.ja.normalize_neologd("      おお"), "おお")
        self.assertEqual(yotsuba.ja.normalize_neologd("おお      "), "おお")
        self.assertEqual(
            yotsuba.ja.normalize_neologd("検索 エンジン 自作 入門 を 買い ました!!!"),
            "検索エンジン自作入門を買いました!!!"
        )
        self.assertEqual(
            yotsuba.ja.normalize_neologd("アルゴリズム C"), "アルゴリズムC"
        )
        self.assertEqual(
            yotsuba.ja.normalize_neologd("　　　ＰＲＭＬ　　副　読　本　　　"), "PRML副読本"
        )
        self.assertEqual(
            yotsuba.ja.normalize_neologd("Coding the Matrix"),
            "Coding the Matrix"
        )
        self.assertEqual(
            yotsuba.ja.normalize_neologd("南アルプスの　天然水　Ｓｐａｒｋｉｎｇ　Ｌｅｍｏｎ　レモン一絞り"),
            "南アルプスの天然水Sparking Lemonレモン一絞り"
        )
        # self.assertEqual(
        #     '南アルプスの天然水-Sparking*Lemon+レモン一絞り',
        #     yotsuba.ja.normalize_neologd(
        #         '南アルプスの　天然水-　Ｓｐａｒｋｉｎｇ*　Ｌｅｍｏｎ+　レモン一絞り'
        #     )
        # )
        processing_time: float = time.time() - start
        print(f'Processing time of rust   : {processing_time}')


class PadSequenceTests(unittest.TestCase):

    def test_simple(self):
        padding: List[int] = yotsuba.pad_sequence(
            sequence=[0, 2, 1], maxlen=5, value=-1, padding='post'
        )
        self.assertEqual(padding, [0, 2, 1, -1, -1])

    def test_padding_pre(self):
        padding: List[int] = yotsuba.pad_sequence(
            sequence=[0, 2, 1], maxlen=5, value=-1, padding='pre'
        )
        self.assertEqual(padding, [-1, -1, 0, 2, 1])

    def test_option_value(self):
        padding: List[int] = yotsuba.pad_sequence(
            sequence=[0, 2, 1], maxlen=5, padding='post'
        )
        self.assertEqual(padding, [0, 2, 1, 0, 0])

    def test_raise_invalid_padding(self):
        with self.assertRaises(ValueError):
            yotsuba.pad_sequence(
                sequence=[0, 2, 1], maxlen=5, padding='invalid-padding'
            )


class PadSequencesTests(unittest.TestCase):

    def test_simple(self):
        sequences: List[List[int]] = [[0, 2, 1], [0, 1]]
        padding: List[List[int]] = yotsuba.pad_sequences(
            sequences=sequences, maxlen=5, value=-1, padding='post'
        )
        self.assertEqual(padding, [[0, 2, 1, -1, -1], [0, 1, -1, -1, -1]])

    def test_option_maxlen(self):
        sequences: List[List[int]] = [[0, 2, 1], [0, 1]]
        padding: List[List[int]] = yotsuba.pad_sequences(
            sequences=sequences, value=-1, padding='post'
        )
        self.assertEqual(padding, [[0, 2, 1], [0, 1, -1]])

    def test_option_value(self):
        sequences: List[List[int]] = [[0, 2, 1], [0, 1]]
        padding: List[List[int]] = yotsuba.pad_sequences(
            sequences=sequences, padding='post'
        )
        self.assertEqual(padding, [[0, 2, 1], [0, 1, 0]])

    def test_raise_invalid_padding(self):
        sequences: List[List[int]] = [[0, 2, 1], [0, 1]]
        with self.assertRaises(ValueError):
            yotsuba.pad_sequences(
                sequences=sequences, padding='invalid'
            )


class RemoveStopwordsTests(unittest.TestCase):

    def test_simple(self):
        tokens: List[str] = ['I', 'am', 'a', 'dog']
        stopwords: List[str] = ['am', 'a']
        removed: List[str] = yotsuba.remove_stopwords(
            tokens, stopwords
        )
        self.assertEqual(removed, ['I', 'dog'])


class GetStopwordsTests(unittest.TestCase):

    def test_simple(self):
        stopwords: List[str] = yotsuba.get_stopwords(lang='ja')
        self.assertIsInstance(stopwords, list)
        self.assertEqual(len(stopwords), 310)


class GetStopwordsByFrequencyTests(unittest.TestCase):

    def test_simple(self):
        tokens: List[List[str]] = [
            ['I', 'am', 'a', 'pen', 'pen'] for _ in range(50)
        ]
        stopwords: List[str] = yotsuba.get_stopwords_by_frequency(tokens, 100)
        self.assertEqual(stopwords, ['pen'])


class CleanURLTests(unittest.TestCase):

    def test_simple(self):
        text: str = 'foohttp://example.com bar'
        cleaned: str = yotsuba.clean_url(
            text, ''
        )
        self.assertEqual(cleaned, 'foo bar')

    def test_replace(self):
        text: str = 'foohttp://example.com bar'
        cleaned: str = yotsuba.clean_url(
            text=text, replace='<URL>'
        )
        self.assertEqual(cleaned, 'foo<URL> bar')


if __name__ == '__main__':
    unittest.main()
