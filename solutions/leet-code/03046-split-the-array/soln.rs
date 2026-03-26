impl Solution {
    pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
        let mut ans = vec![0;101];
        nums.into_iter().for_each(|x| ans[x as usize]+=1);
        ans.into_iter().all(|x| x<=2)
    }
}
