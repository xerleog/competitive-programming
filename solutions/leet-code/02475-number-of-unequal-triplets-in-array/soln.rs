use std::collections::HashMap;
impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
    let mut count = HashMap::new();
    for i in nums {
        *count.entry(i).or_insert(0) += 1;
    }
    
    let unique_nums: Vec<_> = count.keys().cloned().collect();
    let n = unique_nums.len();
    let mut total_triplets = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                let a = unique_nums[i];
                let b = unique_nums[j];
                let c = unique_nums[k];
                total_triplets += count[&a] * count[&b] * count[&c] as u64;
            }
        }
    }

    total_triplets as i32

    }
}
