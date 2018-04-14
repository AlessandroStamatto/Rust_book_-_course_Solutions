/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let mut marked = [false; 2048];
    let mut res = vec![];
    
    for i in 2..n {
    	if !marked[i as usize] {
    		res.push(i);
    	}
    	let (mut mult, mut factor) = (i*2, 2);
    	while mult < n {
    		marked[mult as usize] = true;
    		factor += 1;
    		mult = i*factor;
    	}
    }
    res
}