impl Solution {
    pub fn min_subsequence(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by(|a,b| b.cmp(&a));
        let ma = nums.clone().into_iter().sum::<i32>();
        let (mut ans,mut mi) = (vec![],0);
        for i in nums
        {
            mi+=i;
            ans.push(i);  
            if mi>ma-mi
            {
                break;
            }
        }
        ans
    }
}
