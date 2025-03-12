impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let (a,b) = nums.into_iter().fold((0,0),|(p,n),x| {if x>0 { (p+1,n)} else if x<0 { (p,n+1)} else { (p,n)}});
        a.max(b)
    }
}
