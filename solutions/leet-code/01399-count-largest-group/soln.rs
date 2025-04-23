impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let (mut f, mut gmax, mut cnt) = ([0; 37], 0, 0);
        for x in 1..=n {
            let (mut s, mut y) = (0, x);
            while y > 0 { s += y as usize % 10; y /= 10 }
            f[s] += 1; let g = f[s];
            if g > gmax { gmax = g; cnt = 1 }
            else if g == gmax { cnt += 1 }
        } cnt
    }
}
