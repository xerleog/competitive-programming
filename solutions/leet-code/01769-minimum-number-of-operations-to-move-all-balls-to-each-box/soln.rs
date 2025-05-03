impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let mut ans = vec![0;boxes.len()];
        for i in 0..boxes.len()
        {
            for (j,k) in boxes.chars().enumerate()
            {
                if k=='1' && i!=j
                {
                    ans[i]+=((i-j) as i32).abs();
                }
            }
        }
        ans
    }
}
