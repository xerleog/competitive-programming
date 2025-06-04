impl Solution {
    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        vec![nums1.clone().into_iter().filter(|x| nums2.contains(x)).count() as i32,nums2.into_iter().filter(|x| nums1.contains(x)).count() as i32]
    }
}
