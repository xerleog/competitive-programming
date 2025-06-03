impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        (1..nums.len()+1).filter(|x| nums.len()%x==0).map(|y| nums[y-1]*nums[y-1]).sum::<i32>()
    }
}
