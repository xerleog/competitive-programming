impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {

        let (a,b) = nums.into_iter()
            .fold((i32::MAX, i32::MIN), |(a,b),x|(a.min(x), b.max(x)));
        let m = (a+b)/2;
        b - a - k.min(m-a) - k.min(b-m)
    }
}
