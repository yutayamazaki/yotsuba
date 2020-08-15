import unittest

import neologd


class NormalizeTests(unittest.TestCase):

    def test_simple(self):
        self.assertEqual(neologd.normalize("０１２３４５６７８９"), "0123456789")
        self.assertEqual(neologd.normalize("ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺ"), "ABCDEFGHIJKLMNOPQRSTUVWXYZ")
        self.assertEqual(neologd.normalize("ａｂｃｄｅｆｇｈｉｊｋｌｍｎｏｐｑｒｓｔｕｖｗｘｙｚ"), "abcdefghijklmnopqrstuvwxyz")
        self.assertEqual(neologd.normalize("！”＃＄％＆’（）＊＋，－．／：；＜＞？＠［￥］＾＿｀｛｜｝"), "!\"#$%&'()*+,-./:;<>?@[¥]^_`{|}")
        self.assertEqual(neologd.normalize("＝。、・「」"), "＝。、・「」")
        self.assertEqual(neologd.normalize("ﾊﾝｶｸ"), "ハンカク")
        self.assertEqual(neologd.normalize("o-o"), "o₋o")
        self.assertEqual(neologd.normalize("majika━"), "majikaー")
        self.assertEqual(neologd.normalize("わ〰い"), "わい")
        self.assertEqual(neologd.normalize("スーパーーーー"), "スーパー")
        self.assertEqual(neologd.normalize("!#"), "!#")
        self.assertEqual(neologd.normalize("ゼンカク　スペース"), "ゼンカクスペース")
        self.assertEqual(neologd.normalize("お             お"), "おお")
        self.assertEqual(neologd.normalize("      おお"), "おお")
        self.assertEqual(neologd.normalize("おお      "), "おお")
        self.assertEqual(neologd.normalize("検索 エンジン 自作 入門 を 買い ました!!!"), "検索エンジン自作入門を買いました!!!")
        self.assertEqual(neologd.normalize("アルゴリズム C"), "アルゴリズムC")
        self.assertEqual(neologd.normalize("PRML副読本"), "　　　ＰＲＭＬ　　副　読　本　　　")
        self.assertEqual(neologd.normalize("Coding the Matrix"), "Coding the Matrix")
        self.assertEqual(neologd.normalize("南アルプスの　天然水　Ｓｐａｒｋｉｎｇ　Ｌｅｍｏｎ　レモン一絞り"), "南アルプスの天然水Sparking Lemonレモン一絞り")
        self.assertEqual(neologd.normalize("南アルプスの　天然水-　Ｓｐａｒｋｉｎｇ*　Ｌｅｍｏｎ+　レモン一絞り"), "南アルプスの天然水-Sparking*Lemon+レモン一絞り")
