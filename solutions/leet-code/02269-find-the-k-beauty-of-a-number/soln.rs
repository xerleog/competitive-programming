impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        unsafe {
            num.to_string()
                .as_bytes()
                .windows(k as usize)
                .map(|x| std::str::from_utf8_unchecked(x).parse::<i32>().unwrap())
                .filter(|&x| x != 0 && num % x == 0)
                .count() as i32
        }
    }
}
