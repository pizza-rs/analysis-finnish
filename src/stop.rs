//! Finnish stop words (from Lucene/Snowball project).

use alloc::borrow::Cow;
use alloc::vec::Vec;
use hashbrown::HashSet;
use once_cell::sync::Lazy;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

/// Default Finnish stop words sourced from Apache Lucene.
static DEFAULT_STOP_WORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let words: &[&str] = &[
        "ei",
        "eivät",
        "emme",
        "en",
        "et",
        "ette",
        "että",
        "he     heidän heidät heitä  heissä  heistä  heihin heillä  heiltä  heille",
        "hän    hänen  hänet  häntä  hänessä hänestä häneen hänellä häneltä hänelle",
        "itse",
        "ja",
        "joka   jonka         jota   jossa   josta   johon  jolla   jolta   jolle   jona   joksi",
        "jos",
        "jotka  joiden        joita  joissa  joista  joihin joilla  joilta  joille  joina  joiksi",
        "kanssa",
        "ketkä  keiden ketkä  keitä  keissä  keistä  keihin keillä  keiltä  keille  keinä  keiksi",
        "koska",
        "kuin",
        "kuka   kenen kenet   ketä   kenessä kenestä keneen kenellä keneltä kenelle kenenä keneksi",
        "kun",
        "me     meidän meidät meitä  meissä  meistä  meihin meillä  meiltä  meille",
        "mikä   minkä minkä   mitä   missä   mistä   mihin  millä   miltä   mille   minä   miksi",
        "minä   minun  minut  minua  minussa minusta minuun minulla minulta minulle",
        "mitkä",
        "mukaan",
        "mutta",
        "ne     niiden        niitä  niissä  niistä  niihin niillä  niiltä  niille  niinä  niiksi",
        "niin",
        "noin",
        "nuo    noiden        noita  noissa  noista  noihin noilla  noilta  noille  noina  noiksi",
        "nyt",
        "nämä   näiden        näitä  näissä  näistä  näihin näillä  näiltä  näille  näinä  näiksi",
        "ole",
        "olemme",
        "olen",
        "olet",
        "olette",
        "oli",
        "olimme",
        "olin",
        "olisi",
        "olisimme",
        "olisin",
        "olisit",
        "olisitte",
        "olisivat",
        "olit",
        "olitte",
        "olivat",
        "olla",
        "olleet",
        "ollut",
        "on",
        "ovat",
        "poikki",
        "se     sen           sitä   siinä   siitä   siihen sillä   siltä   sille   sinä   siksi",
        "sekä",
        "sillä",
        "sinä   sinun  sinut  sinua  sinussa sinusta sinuun sinulla sinulta sinulle",
        "tai",
        "te     teidän teidät teitä  teissä  teistä  teihin teillä  teiltä  teille",
        "tuo    tuon          tuota  tuossa  tuosta  tuohon tuolla  tuolta  tuolle  tuona  tuoksi",
        "tämä   tämän         tätä   tässä   tästä   tähän  tällä   tältä   tälle   tänä   täksi",
        "vaan",
        "vai",
        "vaikka",
        "yli",
    ];
    words.iter().copied().collect()
});

/// Removes Finnish stop words from the token stream.
#[derive(Clone, Debug)]
pub struct FinnishStopFilter {
    stop_words: HashSet<String>,
}

impl Default for FinnishStopFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl FinnishStopFilter {
    pub fn new() -> Self {
        Self {
            stop_words: DEFAULT_STOP_WORDS.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn with_words(words: &[&str]) -> Self {
        Self {
            stop_words: words.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl TokenFilter for FinnishStopFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let term = token.term.as_ref();
        if self.stop_words.contains(term) {
            return (true, None);
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_word_count() {
        assert!(DEFAULT_STOP_WORDS.len() >= 67);
    }

    #[test]
    fn test_filters_stop_word() {
        let f = FinnishStopFilter::new();
        let word = DEFAULT_STOP_WORDS.iter().next().unwrap();
        let mut token = Token::new(word, 0, word.len() as u32, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }

    #[test]
    fn test_passes_non_stop_word() {
        let f = FinnishStopFilter::new();
        let mut token = Token::new("xyzzy_not_a_stop_word", 0, 21, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted);
    }

    #[test]
    fn test_custom_words() {
        let f = FinnishStopFilter::with_words(&["custom", "words"]);
        let mut token = Token::new("custom", 0, 6, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }
}
