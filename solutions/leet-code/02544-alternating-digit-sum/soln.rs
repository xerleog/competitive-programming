impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        n.to_string().as_bytes().chunks(2).map(|x| if x.len() == 2 { x[0] as i32- x[1] as i32 } else { (x[0]-48) as i32}).sum() 
    }
}
