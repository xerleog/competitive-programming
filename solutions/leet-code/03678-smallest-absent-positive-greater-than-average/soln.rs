impl Solution {
    pub fn smallest_absent(nums: Vec<i32>) -> i32 {
        let mut ans=0.max(nums.clone().into_iter().sum::<i32>()/nums.len() as i32)+1;
       while nums.contains(&ans) { ans+=1;}
        ans
    }
}
