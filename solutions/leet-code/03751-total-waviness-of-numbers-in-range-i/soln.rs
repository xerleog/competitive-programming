impl Solution {
    pub fn total_waviness(num1: i32, num2: i32) -> i32 {
        (num1..=num2).map(get_waviness).sum()
    }
}

fn get_waviness(n: i32) -> i32 {
    n.to_string().as_bytes().windows(3).filter(is_peak_or_valley).count() as i32 
}

fn is_peak_or_valley(c: &&[u8]) -> bool {
    (c[0] > c[1] && c[1] < c[2]) || (c[0] < c[1] && c[1] > c[2])
}
