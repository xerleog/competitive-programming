impl Solution {
    pub fn count_opposite_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        for i in 0..nums.len()
        {
            let mut temp = 0;
            for j in i+1..nums.len()
            {
                if nums[i]&1!=nums[j]&1 { temp+=1;}
            }
            ans.push(temp);
        }   
        ans
    }
}
