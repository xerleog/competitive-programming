impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        nums.into_iter()
            .map(|num| num as i64)
            .scan((0, 0), |(max, md), num| {
                *max = num.max(*max);
                Some(std::mem::replace(md, (*md).max(*max - num)) * num)
            })
            .max()
            .unwrap()   
    }
}
