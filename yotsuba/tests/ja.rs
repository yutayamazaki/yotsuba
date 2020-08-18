#[cfg(test)]
mod tests {
    use yotsuba::ja;
    #[test]
    fn is_katakana_works() {
        let katakanas = "アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヰヱヲガギグゲゴザジズゼゾダヂヅデドバビブベボパピプペャュョァィゥェォヴッン";
        for c in katakanas.chars() {
            assert_eq!(ja::is_katakana(&c), true);
        }
        assert_eq!(ja::is_katakana(&'あ'), false);
        assert_eq!(ja::is_katakana(&'1'), false);
    }
}
