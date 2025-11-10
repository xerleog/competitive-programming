impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nge = [-1;10001];
        let mut greater = [-1; 10001];
        for &n in nums2.iter().rev() {
            nge[n as usize] = greater[n as usize];
            for i in 0..n { greater[i as usize] = n; }
        }
        nums1.into_iter().map(|n| nge[n as usize]).collect()
    }
}
