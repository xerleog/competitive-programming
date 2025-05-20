impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let mut ans = vec![0;nums.len()];
        queries.into_iter().for_each(|x| (x[0]..=x[1]).into_iter().for_each(|y| ans[y as usize]+=1));
        nums.into_iter().zip(ans.into_iter()).map(|x| x.0-x.1).all(|y| y<1)
    }
}
