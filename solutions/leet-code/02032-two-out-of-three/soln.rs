impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0_i32;101];
        nums1.into_iter().for_each(|x|  ans[x as usize]|=0b001);
        nums2.into_iter().for_each(|x|  ans[x as usize]|=0b010);
        nums3.into_iter().for_each(|x|  ans[x as usize]|=0b100);
        ans.iter()
            .cloned()
            .enumerate()
            .filter_map(|(ind, x)| match x.count_ones() >= 2 {
                true => Some(ind as i32),
                false => None,
            }).collect::<Vec<_>>()
    }
}
