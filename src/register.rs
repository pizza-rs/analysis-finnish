use alloc::boxed::Box;
use alloc::vec;
use pizza_engine::analysis::AnalysisFactory;
use pizza_engine::analysis::Analyzer;
use pizza_engine::analysis::StandardTokenizer;
use pizza_engine::analysis::TokenFilter;

use crate::stem::FinnishLightStemFilter;
use crate::stop::FinnishStopFilter;

pub fn register_all(factory: &mut AnalysisFactory) {
    factory.register_token_filter("finnish_light_stem", Box::new(FinnishLightStemFilter::new()));
    factory.register_token_filter("finnish_stop", Box::new(FinnishStopFilter::new()));

    let filters: Vec<Box<dyn TokenFilter>> = vec![
        Box::new(FinnishStopFilter::new()),
        Box::new(FinnishLightStemFilter::new()),
    ];

    factory.register_analyzer(
        "finnish",
        Analyzer::new(vec![], Box::new(StandardTokenizer::new()), filters),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_all_no_panic() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
    }

    #[test]
    fn test_filters_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_token_filter("finnish_light_stem").is_some());
        assert!(factory.get_token_filter("finnish_stop").is_some());
    }

    #[test]
    fn test_analyzer_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_analyzer("finnish").is_some());
    }
}
