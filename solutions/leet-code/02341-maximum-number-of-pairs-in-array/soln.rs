impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0;101];
        nums.clone().into_iter().for_each(|x| ans[x as usize]+=1);
        let val = ans.into_iter().map(|x| x/2).sum::<i32>();
        vec![val, nums.len() as i32-(2*val)]
    }
}
