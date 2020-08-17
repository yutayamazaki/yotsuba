import unittest

import yotsuba


class NormalizeTests(unittest.TestCase):

    def test_simple(self):
        print(dir(yotsuba))
        self.assertEqual(yotsuba.normalize("０１２３４５６７８９"), "0123456789")
        self.assertEqual(yotsuba.normalize("ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺ"), "ABCDEFGHIJKLMNOPQRSTUVWXYZ")
        self.assertEqual(yotsuba.normalize("ａｂｃｄｅｆｇｈｉｊｋｌｍｎｏｐｑｒｓｔｕｖｗｘｙｚ"), "abcdefghijklmnopqrstuvwxyz")
        self.assertEqual(yotsuba.normalize("！”＃＄％＆’（）＊＋，－．／：；＜＞？＠［￥］＾＿｀｛｜｝"), "!\"#$%&'()*+,-./:;<>?@[¥]^_`{|}")
        self.assertEqual(yotsuba.normalize("＝。、・「」"), "＝。、・「」")
        self.assertEqual(yotsuba.normalize("ﾊﾝｶｸ"), "ハンカク")
        self.assertEqual(yotsuba.normalize("o-o"), "o₋o")
        self.assertEqual(yotsuba.normalize("majika━"), "majikaー")
        self.assertEqual(yotsuba.normalize("わ〰い"), "わい")
        self.assertEqual(yotsuba.normalize("スーパーーーー"), "スーパー")
        self.assertEqual(yotsuba.normalize("!#"), "!#")
        self.assertEqual(yotsuba.normalize("ゼンカク　スペース"), "ゼンカクスペース")
        self.assertEqual(yotsuba.normalize("お             お"), "おお")
        self.assertEqual(yotsuba.normalize("      おお"), "おお")
        self.assertEqual(yotsuba.normalize("おお      "), "おお")
        self.assertEqual(yotsuba.normalize("検索 エンジン 自作 入門 を 買い ました!!!"), "検索エンジン自作入門を買いました!!!")
        self.assertEqual(yotsuba.normalize("アルゴリズム C"), "アルゴリズムC")
        self.assertEqual(yotsuba.normalize("PRML副読本"), "　　　ＰＲＭＬ　　副　読　本　　　")
        self.assertEqual(yotsuba.normalize("Coding the Matrix"), "Coding the Matrix")
        self.assertEqual(yotsuba.normalize("南アルプスの　天然水　Ｓｐａｒｋｉｎｇ　Ｌｅｍｏｎ　レモン一絞り"), "南アルプスの天然水Sparking Lemonレモン一絞り")
        self.assertEqual(yotsuba.normalize("南アルプスの　天然水-　Ｓｐａｒｋｉｎｇ*　Ｌｅｍｏｎ+　レモン一絞り"), "南アルプスの天然水-Sparking*Lemon+レモン一絞り")
