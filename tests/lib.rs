extern crate test_box;

#[test]
fn filter_and_sum_test_tests() {
    assert_eq!(2, test_box::filter_and_sum(3, 2));
}