impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut ans = vec![0;101];
        nums.into_iter().for_each(|x| ans[x as usize]+=1);
        ans.into_iter().enumerate().filter(|(a,b)| *b==1).map(|(a,_)| a as i32).sum::<i32>()
    }
}
