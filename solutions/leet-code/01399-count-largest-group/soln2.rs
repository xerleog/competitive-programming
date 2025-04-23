impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let (mut f, mut gmax) = ([0; 37], 0);
        for x in 1..=n {
            let (mut s, mut y) = (0, x);
            while y > 0 { s += y as usize % 10; y /= 10 }
            f[s] += 1; 
            gmax = gmax.max(f[s]);} 
        f.into_iter().filter(|&x| x==gmax).count() as i32
    }
}
