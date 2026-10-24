//! Composing decomposed Vietnamese letters (NFD, `e` + U+0302 + U+0301) into the precomposed ones
//! (NFC, `ế`) fonts map directly. Vietnamese text often arrives decomposed: macOS file names,
//! some input methods, and text pasted from the web or converted from legacy encodings
//! (Windows-1258 stores the tone as a separate combining mark). A font without mark positioning,
//! like the PDF fonts this crate writes, would otherwise draw the marks beside the letter.
//!
//! The pairs cover every letter of the Vietnamese alphabet with each of the five tone marks and
//! the breve, circumflex and horn; they were generated from Unicode's canonical compositions.

/// Canonical combining class of the marks Vietnamese uses (0 for anything else).
fn ccc(c: char) -> u8 {
    match c {
        '\u{0300}' | '\u{0301}' | '\u{0302}' | '\u{0303}' | '\u{0306}' | '\u{0309}' => 230,
        '\u{0323}' => 220,
        '\u{031B}' => 216,
        _ => 0,
    }
}

/// The precomposed letter for `base` followed by `mark`, if Unicode has one.
fn compose(base: char, mark: char) -> Option<char> {
    PAIRS.binary_search_by(|(b, m, _)| (*b, *m).cmp(&(base, mark))).ok().and_then(|i| PAIRS.get(i)).map(|(_, _, c)| *c)
}

/// `text` with decomposed Vietnamese letters composed. Text without the combining marks
/// Vietnamese uses is returned unchanged (borrowed).
pub fn compose_vietnamese(text: &str) -> std::borrow::Cow<'_, str> {
    if !text.chars().any(|c| ccc(c) != 0) {
        return std::borrow::Cow::Borrowed(text);
    }
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    let mut marks: Vec<char> = Vec::new();
    while let Some(c) = chars.next() {
        if ccc(c) != 0 {
            // A mark with no letter before it (start of text, after a space): kept as is.
            out.push(c);
            continue;
        }
        marks.clear();
        while let Some(m) = chars.peek().copied().filter(|m| ccc(*m) != 0) {
            marks.push(m);
            chars.next();
        }
        // Canonical order (stable by class), then compose each mark that isn't blocked by an
        // earlier uncomposed mark of the same class (Unicode's composition rule).
        marks.sort_by_key(|m| ccc(*m));
        let mut cur = c;
        let mut left: Vec<char> = Vec::new();
        for m in &marks {
            let blocked = left.last().is_some_and(|l| ccc(*l) >= ccc(*m));
            match compose(cur, *m).filter(|_| !blocked) {
                Some(composed) => cur = composed,
                None => left.push(*m),
            }
        }
        out.push(cur);
        out.extend(left);
    }
    std::borrow::Cow::Owned(out)
}

// Sorted by (letter, mark) for binary search.
const PAIRS: &[(char, char, char)] = &[
    ('A', '\u{0300}', 'À'),
    ('A', '\u{0301}', 'Á'),
    ('A', '\u{0302}', 'Â'),
    ('A', '\u{0303}', 'Ã'),
    ('A', '\u{0306}', 'Ă'),
    ('A', '\u{0309}', 'Ả'),
    ('A', '\u{0323}', 'Ạ'),
    ('E', '\u{0300}', 'È'),
    ('E', '\u{0301}', 'É'),
    ('E', '\u{0302}', 'Ê'),
    ('E', '\u{0303}', 'Ẽ'),
    ('E', '\u{0306}', 'Ĕ'),
    ('E', '\u{0309}', 'Ẻ'),
    ('E', '\u{0323}', 'Ẹ'),
    ('I', '\u{0300}', 'Ì'),
    ('I', '\u{0301}', 'Í'),
    ('I', '\u{0302}', 'Î'),
    ('I', '\u{0303}', 'Ĩ'),
    ('I', '\u{0306}', 'Ĭ'),
    ('I', '\u{0309}', 'Ỉ'),
    ('I', '\u{0323}', 'Ị'),
    ('O', '\u{0300}', 'Ò'),
    ('O', '\u{0301}', 'Ó'),
    ('O', '\u{0302}', 'Ô'),
    ('O', '\u{0303}', 'Õ'),
    ('O', '\u{0306}', 'Ŏ'),
    ('O', '\u{0309}', 'Ỏ'),
    ('O', '\u{031B}', 'Ơ'),
    ('O', '\u{0323}', 'Ọ'),
    ('U', '\u{0300}', 'Ù'),
    ('U', '\u{0301}', 'Ú'),
    ('U', '\u{0302}', 'Û'),
    ('U', '\u{0303}', 'Ũ'),
    ('U', '\u{0306}', 'Ŭ'),
    ('U', '\u{0309}', 'Ủ'),
    ('U', '\u{031B}', 'Ư'),
    ('U', '\u{0323}', 'Ụ'),
    ('Y', '\u{0300}', 'Ỳ'),
    ('Y', '\u{0301}', 'Ý'),
    ('Y', '\u{0302}', 'Ŷ'),
    ('Y', '\u{0303}', 'Ỹ'),
    ('Y', '\u{0309}', 'Ỷ'),
    ('Y', '\u{0323}', 'Ỵ'),
    ('a', '\u{0300}', 'à'),
    ('a', '\u{0301}', 'á'),
    ('a', '\u{0302}', 'â'),
    ('a', '\u{0303}', 'ã'),
    ('a', '\u{0306}', 'ă'),
    ('a', '\u{0309}', 'ả'),
    ('a', '\u{0323}', 'ạ'),
    ('e', '\u{0300}', 'è'),
    ('e', '\u{0301}', 'é'),
    ('e', '\u{0302}', 'ê'),
    ('e', '\u{0303}', 'ẽ'),
    ('e', '\u{0306}', 'ĕ'),
    ('e', '\u{0309}', 'ẻ'),
    ('e', '\u{0323}', 'ẹ'),
    ('i', '\u{0300}', 'ì'),
    ('i', '\u{0301}', 'í'),
    ('i', '\u{0302}', 'î'),
    ('i', '\u{0303}', 'ĩ'),
    ('i', '\u{0306}', 'ĭ'),
    ('i', '\u{0309}', 'ỉ'),
    ('i', '\u{0323}', 'ị'),
    ('o', '\u{0300}', 'ò'),
    ('o', '\u{0301}', 'ó'),
    ('o', '\u{0302}', 'ô'),
    ('o', '\u{0303}', 'õ'),
    ('o', '\u{0306}', 'ŏ'),
    ('o', '\u{0309}', 'ỏ'),
    ('o', '\u{031B}', 'ơ'),
    ('o', '\u{0323}', 'ọ'),
    ('u', '\u{0300}', 'ù'),
    ('u', '\u{0301}', 'ú'),
    ('u', '\u{0302}', 'û'),
    ('u', '\u{0303}', 'ũ'),
    ('u', '\u{0306}', 'ŭ'),
    ('u', '\u{0309}', 'ủ'),
    ('u', '\u{031B}', 'ư'),
    ('u', '\u{0323}', 'ụ'),
    ('y', '\u{0300}', 'ỳ'),
    ('y', '\u{0301}', 'ý'),
    ('y', '\u{0302}', 'ŷ'),
    ('y', '\u{0303}', 'ỹ'),
    ('y', '\u{0309}', 'ỷ'),
    ('y', '\u{0323}', 'ỵ'),
    ('Â', '\u{0300}', 'Ầ'),
    ('Â', '\u{0301}', 'Ấ'),
    ('Â', '\u{0303}', 'Ẫ'),
    ('Â', '\u{0309}', 'Ẩ'),
    ('Â', '\u{0323}', 'Ậ'),
    ('Ê', '\u{0300}', 'Ề'),
    ('Ê', '\u{0301}', 'Ế'),
    ('Ê', '\u{0303}', 'Ễ'),
    ('Ê', '\u{0309}', 'Ể'),
    ('Ê', '\u{0323}', 'Ệ'),
    ('Ò', '\u{031B}', 'Ờ'),
    ('Ó', '\u{031B}', 'Ớ'),
    ('Ô', '\u{0300}', 'Ồ'),
    ('Ô', '\u{0301}', 'Ố'),
    ('Ô', '\u{0303}', 'Ỗ'),
    ('Ô', '\u{0309}', 'Ổ'),
    ('Ô', '\u{0323}', 'Ộ'),
    ('Õ', '\u{0301}', 'Ṍ'),
    ('Õ', '\u{031B}', 'Ỡ'),
    ('Ù', '\u{031B}', 'Ừ'),
    ('Ú', '\u{031B}', 'Ứ'),
    ('â', '\u{0300}', 'ầ'),
    ('â', '\u{0301}', 'ấ'),
    ('â', '\u{0303}', 'ẫ'),
    ('â', '\u{0309}', 'ẩ'),
    ('â', '\u{0323}', 'ậ'),
    ('ê', '\u{0300}', 'ề'),
    ('ê', '\u{0301}', 'ế'),
    ('ê', '\u{0303}', 'ễ'),
    ('ê', '\u{0309}', 'ể'),
    ('ê', '\u{0323}', 'ệ'),
    ('ò', '\u{031B}', 'ờ'),
    ('ó', '\u{031B}', 'ớ'),
    ('ô', '\u{0300}', 'ồ'),
    ('ô', '\u{0301}', 'ố'),
    ('ô', '\u{0303}', 'ỗ'),
    ('ô', '\u{0309}', 'ổ'),
    ('ô', '\u{0323}', 'ộ'),
    ('õ', '\u{0301}', 'ṍ'),
    ('õ', '\u{031B}', 'ỡ'),
    ('ù', '\u{031B}', 'ừ'),
    ('ú', '\u{031B}', 'ứ'),
    ('Ă', '\u{0300}', 'Ằ'),
    ('Ă', '\u{0301}', 'Ắ'),
    ('Ă', '\u{0303}', 'Ẵ'),
    ('Ă', '\u{0309}', 'Ẳ'),
    ('Ă', '\u{0323}', 'Ặ'),
    ('ă', '\u{0300}', 'ằ'),
    ('ă', '\u{0301}', 'ắ'),
    ('ă', '\u{0303}', 'ẵ'),
    ('ă', '\u{0309}', 'ẳ'),
    ('ă', '\u{0323}', 'ặ'),
    ('Ũ', '\u{0301}', 'Ṹ'),
    ('Ũ', '\u{031B}', 'Ữ'),
    ('ũ', '\u{0301}', 'ṹ'),
    ('ũ', '\u{031B}', 'ữ'),
    ('Ơ', '\u{0300}', 'Ờ'),
    ('Ơ', '\u{0301}', 'Ớ'),
    ('Ơ', '\u{0303}', 'Ỡ'),
    ('Ơ', '\u{0309}', 'Ở'),
    ('Ơ', '\u{0323}', 'Ợ'),
    ('ơ', '\u{0300}', 'ờ'),
    ('ơ', '\u{0301}', 'ớ'),
    ('ơ', '\u{0303}', 'ỡ'),
    ('ơ', '\u{0309}', 'ở'),
    ('ơ', '\u{0323}', 'ợ'),
    ('Ư', '\u{0300}', 'Ừ'),
    ('Ư', '\u{0301}', 'Ứ'),
    ('Ư', '\u{0303}', 'Ữ'),
    ('Ư', '\u{0309}', 'Ử'),
    ('Ư', '\u{0323}', 'Ự'),
    ('ư', '\u{0300}', 'ừ'),
    ('ư', '\u{0301}', 'ứ'),
    ('ư', '\u{0303}', 'ữ'),
    ('ư', '\u{0309}', 'ử'),
    ('ư', '\u{0323}', 'ự'),
    ('Ạ', '\u{0302}', 'Ậ'),
    ('Ạ', '\u{0306}', 'Ặ'),
    ('ạ', '\u{0302}', 'ậ'),
    ('ạ', '\u{0306}', 'ặ'),
    ('Ẹ', '\u{0302}', 'Ệ'),
    ('ẹ', '\u{0302}', 'ệ'),
    ('Ọ', '\u{0302}', 'Ộ'),
    ('Ọ', '\u{031B}', 'Ợ'),
    ('ọ', '\u{0302}', 'ộ'),
    ('ọ', '\u{031B}', 'ợ'),
    ('Ỏ', '\u{031B}', 'Ở'),
    ('ỏ', '\u{031B}', 'ở'),
    ('Ụ', '\u{031B}', 'Ự'),
    ('ụ', '\u{031B}', 'ự'),
    ('Ủ', '\u{031B}', 'Ử'),
    ('ủ', '\u{031B}', 'ử'),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_decomposed_vietnamese() {
        // Tiếng Việt, decomposed with the marks in canonical order (dot below before circumflex).
        assert_eq!(compose_vietnamese("Tie\u{0302}\u{0301}ng Vie\u{0323}\u{0302}t"), "Tiếng Việt");
        // Marks typed in the "wrong" order still compose (canonical reordering).
        assert_eq!(compose_vietnamese("Vie\u{0302}\u{0323}t"), "Việt");
        // Horn and tone: ườ, ỡ; capital letters; breve with tone: Ặ.
        assert_eq!(compose_vietnamese("u\u{031B}o\u{031B}\u{0300} o\u{031B}\u{0303} A\u{0306}\u{0323}"), "ườ ỡ Ặ");
        // Windows-1258 style: a precomposed vowel followed by a combining tone mark.
        assert_eq!(compose_vietnamese("vi\u{00EA}\u{0323}t"), "việt");
    }

    #[test]
    fn leaves_other_text_alone() {
        assert!(matches!(compose_vietnamese("café ăn đi"), std::borrow::Cow::Borrowed(_)));
        // A stray mark at the start, and a mark no letter takes, stay.
        assert_eq!(compose_vietnamese("\u{0301}x"), "\u{0301}x");
        assert_eq!(compose_vietnamese("q\u{0323}"), "q\u{0323}");
        // Two tone marks: the second one is left uncomposed after the first.
        assert_eq!(compose_vietnamese("a\u{0301}\u{0300}"), "á\u{0300}");
    }

    #[test]
    fn table_is_sorted() {
        assert!(PAIRS.windows(2).all(|w| (w[0].0, w[0].1) < (w[1].0, w[1].1)));
    }
}
