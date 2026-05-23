<div align="center">

# 🇫🇮 pizza-analysis-finnish

**Finnish text analysis plugin for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--finnish-blue)](https://github.com/pizza-rs/analysis-finnish)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## Overview

Finnish language analysis with a light stemmer designed for Finnish agglutinative
morphology. Finnish has 15 noun cases and extensive suffixation, making light stemming
preferable to avoid over-reduction.

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| TokenFilter | `finnish_light_stem` | Light stemmer for Finnish (suffix stripping) |
| TokenFilter | `finnish_stop` | Finnish stop words (67 entries) |
| Analyzer | `finnish` | Full pipeline: lowercase → light_stem → stop |

## Example

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_finnish::register_all(&mut factory);

let analyzer = factory.get_analyzer("finnish").unwrap();
// "taloissa" (in houses) → "talo" (house)
```

## Installation

```toml
[dependencies]
pizza-analysis-finnish = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["finnish"] }
```

## License

MIT

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>
