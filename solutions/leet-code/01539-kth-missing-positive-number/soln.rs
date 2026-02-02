impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, mut k: i32) -> i32 {
        let (mut i, mut val) = (0, 0);
        loop {
            val += 1;
            match i < arr.len() {
                true if val == arr[i] => i += 1,
                _ => {
                    k -= 1;
                    if k == 0 {
                        break val;
                    }
                }
            }
        }
    }
}
