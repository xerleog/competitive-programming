impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![0;1001];
        nums1.into_iter().for_each(|x| ans[x[0] as usize]+=x[1]);
        nums2.into_iter().for_each(|x| ans[x[0] as usize]+=x[1]);
        ans.into_iter().enumerate().filter(|(a,b)| *b!=0).map(|(a,b)| vec![a as i32,b as i32]).collect::<Vec<_>>()
    }
}
