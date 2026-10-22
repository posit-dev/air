//! The corpus. One `#[test]` per input file, so `nextest` filters read as
//! `corpus::expressions::extract`.

mod corpus {
    air_r_test_macros::gen_corpus_tests!("tests/corpus", air_r_parser_tests::run_file);
}
