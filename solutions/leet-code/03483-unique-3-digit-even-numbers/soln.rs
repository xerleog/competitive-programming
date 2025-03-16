fn check(mut x:i32,f:&[i32]) -> bool{
    let mut g = [0_i32;10];
    while x>0
    {
        g[(x%10)as usize]+=1;
        x/=10;
    }
    (0..=9).all(|i| g[i]<=f[i])
}
impl Solution {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let f = digits.into_iter()
            .fold(vec![0_i32; 10], |mut f, d|{
                f[d as usize] += 1;
                f
            });
        (100..=999).step_by(2).filter(|x|{
            check(*x, &f)
        }).count() as _
    }
}
