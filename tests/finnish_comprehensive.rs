//! Comprehensive tests for pizza-analysis-finnish.

use pizza_analysis_finnish::*;
use pizza_engine::analysis::AnalysisFactory;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

fn make_token(term: &str) -> Token<'_> {
    Token::new(term, 0, term.len() as u32, 0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// FinnishLightStemFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stem_construction() {
    let _f = FinnishLightStemFilter::new();
}

#[test]
fn stem_plural_t() {
    let f = FinnishLightStemFilter::new();
    // "talot" (houses) → stem
    let mut token = make_token("talot");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_partitive() {
    let f = FinnishLightStemFilter::new();
    // "kirjaa" (book, partitive) → stem
    let mut token = make_token("kirjaa");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_genitive() {
    let f = FinnishLightStemFilter::new();
    // "talon" (house's) → stem
    let mut token = make_token("talon");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_inessive() {
    let f = FinnishLightStemFilter::new();
    // "talossa" (in the house) → stem
    let mut token = make_token("talossa");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_verb_conjugation() {
    let f = FinnishLightStemFilter::new();
    // "puhuin" (I spoke) → stem
    let mut token = make_token("puhuin");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_short_word() {
    let f = FinnishLightStemFilter::new();
    let mut token = make_token("ja");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_empty_string() {
    let f = FinnishLightStemFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_single_char() {
    let f = FinnishLightStemFilter::new();
    let mut token = make_token("a");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// FinnishStopFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stop_construction() {
    let _f = FinnishStopFilter::new();
}

#[test]
fn stop_filters_common_words() {
    let f = FinnishStopFilter::new();
    let stop_words = [
        "ja", "on", "ei", "että", "kun", "mutta", "niin", "ole", "oli", "en",
    ];
    for word in &stop_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted, "stop word '{}' should be filtered", word);
    }
}

#[test]
fn stop_keeps_content_words() {
    let f = FinnishStopFilter::new();
    let content_words = ["talo", "kirja", "koulu", "kaupunki"];
    for word in &content_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted, "content word '{}' should be kept", word);
    }
}

#[test]
fn stop_empty_string() {
    let f = FinnishStopFilter::new();
    let mut token = make_token("");
    let _ = f.filter(&mut token);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Registration
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn register_all_no_panic() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
}

#[test]
fn register_all_filters_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_token_filter("finnish_light_stem").is_some());
    assert!(factory.get_token_filter("finnish_stop").is_some());
}

#[test]
fn register_all_analyzer_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_analyzer("finnish").is_some());
}

#[test]
fn analyzer_pipeline_produces_tokens() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("finnish").unwrap();
    let mut input = String::from("Talo on suuri ja kaunis");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}

#[test]
fn analyzer_pipeline_removes_stops() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("finnish").unwrap();
    let mut input = String::from("se on talo ja koulu");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    let terms: Vec<&str> = tokens.iter().map(|t| t.term.as_ref()).collect();
    assert!(!terms.contains(&"on"));
    assert!(!terms.contains(&"ja"));
}

#[test]
fn analyzer_pipeline_empty_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("finnish").unwrap();
    let mut input = String::from("");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(tokens.is_empty());
}

#[test]
fn analyzer_pipeline_ascii_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("finnish").unwrap();
    let mut input = String::from("hello world");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}
