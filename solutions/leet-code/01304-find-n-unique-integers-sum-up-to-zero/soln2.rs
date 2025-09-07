impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        (1..=n/2).flat_map(|x| vec![x,-1*x]).chain(if n % 2 != 0 { vec![0] } else { vec![] }).collect()
    }
}
