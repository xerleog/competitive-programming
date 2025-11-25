impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        if k == 1 {return 1;}
        if k % 2 == 0 || k % 5 == 0 {return -1;}
        let (mut t, mut rslt, mut m) = (1,1,1);
        while t % k > 0 {
            m = (m * 10) % k;
            t += m; rslt += 1;
        }
        rslt 
    }
}
