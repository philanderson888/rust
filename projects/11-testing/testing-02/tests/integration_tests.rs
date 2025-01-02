use adder::add_two; 
mod common;
use crate::common::setup;

#[test]
fn integration_test_add_two() {
    setup();
    let result = add_two(2);
    assert_eq!(result, 4);
}
