use std::collections::VecDeque;
impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        const MODULO: i32 = 10_i32.pow(9) + 7;

        const ALPHABET_SIZE: usize = 26;
        let mut ch_counts = VecDeque::from([0; ALPHABET_SIZE]);
        for b in s.into_bytes() {
            ch_counts[(b - b'a') as usize] += 1;
        }

        for _ in 0..t {
            let z_count = ch_counts.pop_back().unwrap();
            ch_counts[0] = (ch_counts[0] + z_count) % MODULO;
            ch_counts.push_front(z_count);
        }

        ch_counts
            .into_iter()
            .reduce(|c1, c2| (c1 + c2) % MODULO)
            .unwrap()
    }
}

