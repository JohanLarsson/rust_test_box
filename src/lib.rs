pub fn filter_and_sum(n: i32, m: i32) -> i32{
	(0..n).filter(|x| x % m == 0)
		  .fold(0, |sum, x| sum + x)
}

#[test]
fn filter_and_sum_test_inline() {
	let sum = filter_and_sum(100, 2);
	assert_eq!(2450, sum);
}
