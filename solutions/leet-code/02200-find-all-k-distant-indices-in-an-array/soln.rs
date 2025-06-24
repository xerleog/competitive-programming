impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let mut ans = vec![];
        for i in 0..nums.len()
        {
            if nums[i]==key
            {
                let i = i as i32;
                for j in i-k..=i+k
                {
                    if j>=0 && j<nums.len() as i32
                    {
                        ans.push(j);
                    }
                }
            }
        }
        ans.sort();
        ans.dedup();
        ans
    }
}
