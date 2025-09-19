impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let sum = nums.clone().into_iter().sum::<i32>();
        let mut dig = 0;
        for i in nums
        {
            let mut temp = i;
            while temp>0
            {
                dig +=temp%10;
                temp/=10;
            }
        }
        (sum-dig).abs()
    }
}
