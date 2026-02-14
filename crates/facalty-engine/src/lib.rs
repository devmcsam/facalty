// baseline lints, general purpose
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
// compatibility lints, will switch to deny in first major release
#![warn(warnings)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(rust_2024_compatibility)]
// code quality lints
#![warn(clippy::module_name_repetitions)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::must_use_candidate)]
#![warn(clippy::return_self_not_must_use)]
#![warn(clippy::wildcard_imports)]
#![warn(clippy::single_match_else)]
#![warn(clippy::similar_names)]
#![warn(clippy::struct_excessive_bools)]
#![warn(clippy::too_many_lines)]
#![warn(clippy::too_many_arguments)]
#![warn(clippy::type_repetition_in_bounds)]
#![warn(clippy::doc_markdown)]
#![warn(clippy::match_bool)]
#![warn(clippy::match_same_arms)]
#![warn(clippy::unnested_or_patterns)]
// denies, mainly just ensure no panics possible
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![allow(clippy::incompatible_msrv)]

mod parsing;
mod types;
