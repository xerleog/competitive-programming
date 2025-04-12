impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let (max,min) = nums.into_iter().fold((0,i32::MAX),|(a,b),x| (a.max(x),b.min(x)));
        for i in 0..min {
            if min % (min - i) == 0 && max % (min - i) == 0 {
                return min - i;
            }
        }
        1
    }
}
