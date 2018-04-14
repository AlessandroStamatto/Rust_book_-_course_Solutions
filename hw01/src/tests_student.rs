#![cfg(test)]

use problem1::{sum, dedup, filter};
use problem2::mat_mult;
use problem3::sieve;
use problem4::{hanoi, Peg};

//My Problem 1 Tests
#[test]
fn test_sum_big() {
    let mut array = vec![];
    for i in 1..101 {
    	array.push(i);
    }
    assert_eq!(sum(&array), 100*(1 + 100)/2);
}

#[test]
fn test_dedup_big() {
    let vs = vec![1,2,2,3,4,1,5,2,3,5,6,7,5,8,9,10];
    assert_eq!(dedup(&vs), vec![1,2,3,4,5,6,7,8,9,10]);
}

fn odd_predicate(x: i32) -> bool {
    (x % 2) == 1
}

#[test]
fn test_filter_big() {
    let vs = vec![1,2,3,4,5,6,7,8,9,10];
    assert_eq!(filter(&vs, &odd_predicate), vec![1,3,5,7,9]);
}


//My Problem 2 Test
#[test]
fn test_mat_mult_associative() {
    let mat1 = vec![vec![2.;3]; 3];
    let mat2 = vec![vec![5.;3]; 3];
    let result1 = mat_mult(&mat1, &mat2);
    let result2 = mat_mult(&mat2, &mat1);

    for i in 0..result1.len() {
        for j in 0..result1[i].len() {
            assert_eq!(result1[i][j], result2[i][j]);
        }
    }
}

//My Problem 3 Test
#[test]
fn test_sieve_advanced() {
    assert_eq!(vec![2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71], sieve(72));
}

//My Problem 4 Test
#[test]
fn test_hanoi_3_disks() {
    let result = hanoi(3, Peg::A, Peg::B, Peg::C);
    assert_eq!(vec![
    	(Peg::A, Peg::C),
    	(Peg::A, Peg::B),
    	(Peg::C, Peg::B),
    	(Peg::A, Peg::C),
    	(Peg::B, Peg::A),
    	(Peg::B, Peg::C),
    	(Peg::A, Peg::C)
    	], result);
    assert_eq!(7, result.len());
}