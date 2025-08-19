impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let (mut ans,mut count)= (0,0);
        for i in 0..nums.len()
        {
            if nums[i]==0 { count+=1;}
            else
            {
                if count>0
                {
                    ans+=(count*(count+1))/2;
                    count=0;
                }
            }
        }
        if count>0 { ans+=(count*(count+1))/2;}
        ans
    }
}
