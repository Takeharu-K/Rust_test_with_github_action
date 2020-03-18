pub fn add(a: i32, b: i32) -> i32 {
	a + b
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn check_add() {
		assert_eq!(6, add(4, 2));
	}

	#[test]
	fn other_test1() {
		assert!(true);
	}

	#[test]
	#[should_panic]
	fn other_test2() {
		assert!(false)
	}
}