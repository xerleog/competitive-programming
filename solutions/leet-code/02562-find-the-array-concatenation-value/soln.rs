impl Solution {
    pub fn conc(a:i64,b:i64) -> i64 {
       ((a.to_string()) + &(b.to_string())).parse::<i64>().unwrap()
    }
    pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
        let (mut r,mut l) = (0,nums.len()-1);
        let mut ans = 0_i64;
        while r<l
        {
            ans+=Self::conc(nums[r] as i64,nums[l] as i64);
            r+=1;
            l-=1;
        }
        if r==l
        {
            ans+=nums[r] as i64;
        }
        ans
    }
}
