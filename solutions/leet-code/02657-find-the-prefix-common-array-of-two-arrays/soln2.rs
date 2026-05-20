impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let l = a.len();
        let mut rtn = vec![l as i32; l];

        let mut an = 0i64;
        let mut bn = 0i64;

        for i in 0..l - 1 {
            an |= 1i64 << a[i];
            bn |= 1i64 << b[i];
            rtn[i] = (an & bn).count_ones() as i32;
        }

        rtn
    }
}
