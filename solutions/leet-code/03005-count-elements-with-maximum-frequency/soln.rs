impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let (mut ans,mut sol) = (vec![0;101],0);
        nums.into_iter().for_each(|x| {ans[(x-1) as usize]+=1; sol=sol.max(ans[(x-1)as usize])});
        ans.into_iter().filter(|&x| x==sol).count() as i32*sol
    }
}
