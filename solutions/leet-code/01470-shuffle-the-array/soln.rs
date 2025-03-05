impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        nums[0..n as usize].iter().zip(&nums[n as usize..]).flat_map(|(&a,&b)| vec![a,b]).collect::<Vec<_>>()
    }
}
