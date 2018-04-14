/// Computes the sum of all elements in the input i32 slice named `slice`
pub fn sum(slice: &[i32]) -> i32 {
	let mut s: i32 = 0;
	for value in slice.iter() {
		s = s + value;
	}
    s
}

/// Deduplicates items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, preserving the
/// original order.
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
	use std::collections::HashSet;
	let mut nums = HashSet::new();
    let mut v = vec![];
    for &value in vs.iter() {
    	if !nums.contains(&value)  {
    		v.push(value);
    		nums.insert(value);
    	}
    }
    v
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut v = vec![];
    for &value in vs.iter() {
    	if pred(value) {
    		v.push(value);
    	}
    }
    v
}