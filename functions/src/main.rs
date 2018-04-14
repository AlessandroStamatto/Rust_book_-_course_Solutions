fn testinho(x: i64, y: i32) -> i64 {
	x + (y as i64)
}

fn main() {
    println!("{}!", testinho(5, 3));
}
