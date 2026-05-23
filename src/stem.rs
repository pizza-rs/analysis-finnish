use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

/// Finnish light stemmer. Finnish is highly agglutinative;
/// this removes common case endings and plural markers.
#[derive(Clone, Debug, Default)]
pub struct FinnishLightStemFilter;

impl FinnishLightStemFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for FinnishLightStemFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        if text.len() < 5 {
            return (false, None);
        }

        let stemmed = stem_finnish(text);
        if stemmed != text {
            token.term = Cow::Owned(stemmed);
        }
        (false, None)
    }
}

fn stem_finnish(word: &str) -> String {
    let mut s = word.to_string();

    // Step 1: Remove possessive suffixes
    s = step1_possessive(&s);

    // Step 2: Remove case endings
    s = step2_case(&s);

    // Step 3: Remove derivational suffixes
    s = step3_derivational(&s);

    // Normalize: truncate doubled vowels at end
    s = norm_vowels(&s);

    s
}

fn step1_possessive(word: &str) -> String {
    let suffixes: &[&str] = &[
        "mme", "nne", "nsa", "nsä", "ni", "si",
    ];
    for suffix in suffixes {
        if word.ends_with(suffix) {
            let stem_len = word.len() - suffix.len();
            if stem_len >= 4 {
                return word[..stem_len].to_string();
            }
        }
    }
    word.to_string()
}

fn step2_case(word: &str) -> String {
    let suffixes: &[&str] = &[
        // Longest first
        "ista", "istä", "issa", "issä",
        "illa", "illä", "ilta", "iltä",
        "itta", "ittä",
        "iden", "iden",
        "seen",
        "lle", "lta", "ltä", "sta", "stä", "ssa", "ssä",
        "ina", "inä", "ita", "itä",
        "lla", "llä",
        "ksi",
        "ien", "den", "ten",
        "eet",
        "ia", "iä", "ta", "tä", "na", "nä",
        "in", "en", "an", "on", "un", "yn", "ön", "än",
        "it", "at", "et", "ut", "yt", "öt", "ät",
        "ä", "a", "t", "n",
    ];

    for suffix in suffixes {
        if word.ends_with(suffix) {
            let stem_len = word.len() - suffix.len();
            if stem_len >= 4 {
                return word[..stem_len].to_string();
            }
        }
    }
    word.to_string()
}

fn step3_derivational(word: &str) -> String {
    let suffixes: &[&str] = &[
        "llinen", "lliset",
        "lainen", "läinen",
        "inen", "iset",
        "nnut", "nnyt",
        "nut", "nyt", "neet",
        "vat", "vät",
        "isi",
        "ton", "tön",
        "ttain",
        "us", "ys", "ös",
    ];

    for suffix in suffixes {
        if word.ends_with(suffix) {
            let stem_len = word.len() - suffix.len();
            if stem_len >= 4 {
                return word[..stem_len].to_string();
            }
        }
    }
    word.to_string()
}

fn norm_vowels(word: &str) -> String {
    let bytes = word.as_bytes();
    let len = bytes.len();
    if len < 5 {
        return word.to_string();
    }
    // If ends with doubled vowel (aa, ee, ii, oo, uu, ää, öö, yy), trim last
    let chars: Vec<char> = word.chars().collect();
    let clen = chars.len();
    if clen >= 2 {
        let last = chars[clen - 1];
        let prev = chars[clen - 2];
        if last == prev && "aeiouyäö".contains(last) {
            let result: String = chars[..clen - 1].iter().collect();
            if result.len() >= 4 {
                return result;
            }
        }
    }
    word.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stem_case_ending() {
        let filter = FinnishLightStemFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("talossa"),
            start_offset: 0,
            end_offset: 7,
            position: 0,
        };
        let (deleted, _) = filter.filter(&mut token);
        assert!(!deleted);
        assert_eq!(token.term.as_ref(), "talo");
    }

    #[test]
    fn test_short_word_unchanged() {
        let filter = FinnishLightStemFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("ja"),
            start_offset: 0,
            end_offset: 2,
            position: 0,
        };
        let (deleted, _) = filter.filter(&mut token);
        assert!(!deleted);
        assert_eq!(token.term.as_ref(), "ja");
    }
}
