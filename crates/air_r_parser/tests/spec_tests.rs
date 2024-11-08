mod spec_test;

mod ok {
    tests_macros::gen_tests! {"tests/snapshots/ok/**/*.R", crate::spec_test::run, "ok"}

    // TODO: Before enabling the failing tests we must produce diagnostics for
    // bogus and missing nodes

    // tests_macros::gen_tests! {"tests/snapshots/error/**/*.R", crate::spec_test::run, "error"}
}
