mod spec_test;

mod formatter {

    mod r {
        tests_macros::gen_tests! {"tests/specs/r/**/*.R", crate::spec_test::run, "r"}
    }
}
