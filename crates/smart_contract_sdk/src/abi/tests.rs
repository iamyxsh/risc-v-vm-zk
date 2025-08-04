use crate::constants::CONSTRUCTOR_SELECTOR;

use super::*;

#[test]
fn test_selector_consistency() {
    let sig = "foo(u32)";
    let sel = selector(sig);

    assert_eq!(sel, 0x53FC298D);
}

#[test]
fn test_constructor_selector() {
    assert_eq!(CONSTRUCTOR_SELECTOR, 0);
}
