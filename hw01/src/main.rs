pub mod problem2;

fn main() {
	use problem2::mat_mult;
	let mat1 = vec![vec![2.;3]; 3];
    let mat2 = vec![vec![5.;3]; 3];
    let result1 = mat_mult(&mat1, &mat2);
    println!("{:?}", result1);	
}