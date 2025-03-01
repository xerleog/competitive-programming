impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let mut ans =  vec![0;nums.len()];
        (0..nums.len()-1).for_each(|x| { if nums[x]==nums[x+1]{ nums[x]*=2; nums[x+1]=0;} });
        let mut i=0;
        for j in 0..nums.len()
        {
            if nums[j]>0
            {   ans[i]=nums[j];
                i+=1;
            }
        }
        ans
    }
}
