import time
import unittest
from typing import List

import neologdn
import yotsuba

from utils import normalize_neologd


class NormalizeTests(unittest.TestCase):

    def test_rust(self):
        start: float = time.time()
        self.assertEqual(yotsuba.neologd_normalize("０１２３４５６７８９"), "0123456789")
        self.assertEqual(yotsuba.neologd_normalize("ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺ"), "ABCDEFGHIJKLMNOPQRSTUVWXYZ")
        self.assertEqual(yotsuba.neologd_normalize("ａｂｃｄｅｆｇｈｉｊｋｌｍｎｏｐｑｒｓｔｕｖｗｘｙｚ"), "abcdefghijklmnopqrstuvwxyz")
        # self.assertEqual(yotsuba.neologd_normalize("！”＃＄％＆’（）＊＋，－．／：；＜＞？＠［￥］＾＿｀｛｜｝"), "!\"#$%&'()*+,-./:;<>?@[¥]^_`{|}")
        self.assertEqual(yotsuba.neologd_normalize("＝。、・「」"), "＝。、・「」")
        self.assertEqual(yotsuba.neologd_normalize("ﾊﾝｶｸ"), "ハンカク")
        self.assertEqual(yotsuba.neologd_normalize("o₋o"), "o-o")
        self.assertEqual(yotsuba.neologd_normalize("majika━"), "majikaー")
        self.assertEqual(yotsuba.neologd_normalize("わ〰い"), "わい")
        self.assertEqual(yotsuba.neologd_normalize("スーパーーーー"), "スーパー")
        self.assertEqual(yotsuba.neologd_normalize("!#"), "!#")
        self.assertEqual(yotsuba.neologd_normalize("ゼンカク　スペース"), "ゼンカクスペース")
        self.assertEqual(yotsuba.neologd_normalize("お             お"), "おお")
        self.assertEqual(yotsuba.neologd_normalize("      おお"), "おお")
        self.assertEqual(yotsuba.neologd_normalize("おお      "), "おお")
        self.assertEqual(yotsuba.neologd_normalize("検索 エンジン 自作 入門 を 買い ました!!!"), "検索エンジン自作入門を買いました!!!")
        self.assertEqual(yotsuba.neologd_normalize("アルゴリズム C"), "アルゴリズムC")
        self.assertEqual(yotsuba.neologd_normalize("　　　ＰＲＭＬ　　副　読　本　　　"), "PRML副読本")
        self.assertEqual(yotsuba.neologd_normalize("Coding the Matrix"), "Coding the Matrix")
        self.assertEqual(yotsuba.neologd_normalize("南アルプスの　天然水　Ｓｐａｒｋｉｎｇ　Ｌｅｍｏｎ　レモン一絞り"), "南アルプスの天然水Sparking Lemonレモン一絞り")
        # self.assertEqual(yotsuba.neologd_normalize("南アルプスの　天然水-　Ｓｐａｒｋｉｎｇ*　Ｌｅｍｏｎ+　レモン一絞り"), "南アルプスの天然水-Sparking*Lemon+レモン一絞り")
        processing_time: float = time.time() - start
        print(f'Processing time of rust   : {processing_time}')

    def test_python(self):
        start: float = time.time()
        self.assertEqual(normalize_neologd("０１２３４５６７８９"), "0123456789")
        self.assertEqual(normalize_neologd("ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺ"), "ABCDEFGHIJKLMNOPQRSTUVWXYZ")
        self.assertEqual(normalize_neologd("ａｂｃｄｅｆｇｈｉｊｋｌｍｎｏｐｑｒｓｔｕｖｗｘｙｚ"), "abcdefghijklmnopqrstuvwxyz")
        # self.assertEqual(normalize_neologd("！”＃＄％＆’（）＊＋，－．／：；＜＞？＠［￥］＾＿｀｛｜｝"), "!\"#$%&'()*+,-./:;<>?@[¥]^_`{|}")
        self.assertEqual(normalize_neologd("＝。、・「」"), "＝。、・「」")
        self.assertEqual(normalize_neologd("ﾊﾝｶｸ"), "ハンカク")
        self.assertEqual(normalize_neologd("o₋o"), "o-o")
        self.assertEqual(normalize_neologd("majika━"), "majikaー")
        self.assertEqual(normalize_neologd("わ〰い"), "わい")
        self.assertEqual(normalize_neologd("スーパーーーー"), "スーパー")
        self.assertEqual(normalize_neologd("!#"), "!#")
        self.assertEqual(normalize_neologd("ゼンカク　スペース"), "ゼンカクスペース")
        self.assertEqual(normalize_neologd("お             お"), "おお")
        self.assertEqual(normalize_neologd("      おお"), "おお")
        self.assertEqual(normalize_neologd("おお      "), "おお")
        # self.assertEqual(("検索 エンジン 自作 入門 を 買い ました!!!"), "検索エンジン自作入門を買いました!!!")
        self.assertEqual(normalize_neologd("アルゴリズム C"), "アルゴリズムC")
        self.assertEqual(normalize_neologd("　　　ＰＲＭＬ　　副　読　本　　　"), "PRML副読本")
        self.assertEqual(normalize_neologd("Coding the Matrix"), "Coding the Matrix")
        self.assertEqual(normalize_neologd("南アルプスの　天然水　Ｓｐａｒｋｉｎｇ　Ｌｅｍｏｎ　レモン一絞り"), "南アルプスの天然水Sparking Lemonレモン一絞り")
        # self.assertEqual(normalize_neologd("南アルプスの　天然水-　Ｓｐａｒｋｉｎｇ*　Ｌｅｍｏｎ+　レモン一絞り"), "南アルプスの天然水-Sparking*Lemon+レモン一絞り")
        processing_time: float = time.time() - start
        print(f'Processing time of python : {processing_time}')

    def test_neologdn(self):
        start: float = time.time()
        self.assertEqual(neologdn.normalize("０１２３４５６７８９"), "0123456789")
        self.assertEqual(neologdn.normalize("ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺ"), "ABCDEFGHIJKLMNOPQRSTUVWXYZ")
        self.assertEqual(neologdn.normalize("ａｂｃｄｅｆｇｈｉｊｋｌｍｎｏｐｑｒｓｔｕｖｗｘｙｚ"), "abcdefghijklmnopqrstuvwxyz")
        # self.assertEqual(neologdn.normalize("！”＃＄％＆’（）＊＋，－．／：；＜＞？＠［￥］＾＿｀｛｜｝"), "!\"#$%&'()*+,-./:;<>?@[¥]^_`{|}")
        # self.assertEqual(neologdn.normalize("＝。、・「」"), "＝。、・「」")
        self.assertEqual(neologdn.normalize("ﾊﾝｶｸ"), "ハンカク")
        self.assertEqual(neologdn.normalize("o₋o"), "o-o")
        self.assertEqual(neologdn.normalize("majika━"), "majikaー")
        self.assertEqual(neologdn.normalize("わ〰い"), "わい")
        self.assertEqual(neologdn.normalize("スーパーーーー"), "スーパー")
        self.assertEqual(neologdn.normalize("!#"), "!#")
        self.assertEqual(neologdn.normalize("ゼンカク　スペース"), "ゼンカクスペース")
        self.assertEqual(neologdn.normalize("お             お"), "おお")
        self.assertEqual(neologdn.normalize("      おお"), "おお")
        self.assertEqual(neologdn.normalize("おお      "), "おお")
        # self.assertEqual(("検索 エンジン 自作 入門 を 買い ました!!!"), "検索エンジン自作入門を買いました!!!")
        self.assertEqual(neologdn.normalize("アルゴリズム C"), "アルゴリズムC")
        self.assertEqual(neologdn.normalize("　　　ＰＲＭＬ　　副　読　本　　　"), "PRML副読本")
        self.assertEqual(neologdn.normalize("Coding the Matrix"), "Coding the Matrix")
        self.assertEqual(neologdn.normalize("南アルプスの　天然水　Ｓｐａｒｋｉｎｇ　Ｌｅｍｏｎ　レモン一絞り"), "南アルプスの天然水Sparking Lemonレモン一絞り")
        self.assertEqual(normalize_neologd("南アルプスの　天然水-　Ｓｐａｒｋｉｎｇ*　Ｌｅｍｏｎ+　レモン一絞り"), "南アルプスの天然水-Sparking*Lemon+レモン一絞り")
        processing_time: float = time.time() - start
        print(f'Processing time of neologdn: {processing_time}')


class PadSequenceTests(unittest.TestCase):

    def test_simple(self):
        padding: List[int] = yotsuba.pad_sequence(
            sequence=[0, 2, 1], maxlen=5, value=-1
        )
        self.assertEqual(padding, [0, 2, 1, -1, -1])
