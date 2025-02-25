impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let (mut o,mut e)=(0,0);
        let mut ans =0;
        for i in arr
        {
            (o,e) = match i&1{
                0 => (o,e+1),
                _ => (e+1,o),
            };
            ans = (ans+o)%MOD;
        }    
        ans
    }
}
