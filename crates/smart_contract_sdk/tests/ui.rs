#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01.rs");
}
