mod spec_test;

mod ok {
    tests_macros::gen_tests! {"tests/snapshots/ok/**/*.R", crate::spec_test::run, "ok"}
    tests_macros::gen_tests! {"tests/snapshots/error/**/*.R", crate::spec_test::run, "error"}
}
