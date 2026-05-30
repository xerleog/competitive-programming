impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        for i in 0..nums.len()
        {
            let ma = nums[0..i+1].into_iter().max().unwrap();
            let mi = nums[i..].into_iter().min().unwrap();
            if ma-mi <=k { return i as i32;}
        }
        -1
    }
}
