impl Solution {
    pub fn largest_odd_number(mut num: String) -> String {
        while num.len()>0 && num.chars().last().unwrap().to_digit(10).unwrap()%2==0
        {
            num.pop();
        }
        num
    }
}
