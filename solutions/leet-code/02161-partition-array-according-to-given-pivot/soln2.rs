impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut left = vec![];
        let mut right = vec![];
        let mut pivots = vec![];

        for v in nums {
            if v == pivot {
                pivots.push(v);
            } else if v < pivot {
                left.push(v);
            } else {
                right.push(v);
            }
        }
        left.extend(pivots);
        left.extend(right);
        left
    }
}
