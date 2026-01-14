impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        format!("{:b}",n).chars().enumerate().filter(|x| x.1=='1').collect::<Vec<_>>().windows(2).fold(0,|a,c|  a.max((c[1].0-c[0].0) as i32))
    }
}
