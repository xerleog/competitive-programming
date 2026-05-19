use std::collections::HashSet;
impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let nums1: HashSet<_> = nums1.into_iter().collect();
        let nums2: HashSet<_> = nums2.into_iter().collect();
        *nums1.intersection(&nums2).min().unwrap_or(&-1)
    }
}
