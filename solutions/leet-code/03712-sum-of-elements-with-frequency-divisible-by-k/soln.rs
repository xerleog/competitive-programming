impl Solution {
    pub fn sum_divisible_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans :Vec<i32>= vec![0;101];
        nums.into_iter().for_each(|x| ans[x as usize]+=1);
        (0..101).into_iter().map(|x| if x>0 && ans[x]%k==0 { (x as i32)*ans[x]} else { 0}).sum::<i32>()        
    }
}
