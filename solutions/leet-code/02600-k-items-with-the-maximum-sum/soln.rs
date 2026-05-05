impl Solution {
    pub fn k_items_with_maximum_sum(num_ones: i32, num_zeros: i32, num_neg_ones: i32, k: i32) -> i32 {
        let mut ans = vec![1;num_ones as usize];
        ans.extend(vec![0;num_zeros as usize]);
        ans.extend(vec![-1;num_neg_ones as usize]);
        ans.into_iter().take(k as usize).sum::<i32>()
    }
}
