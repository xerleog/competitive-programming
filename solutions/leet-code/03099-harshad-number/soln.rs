impl Solution {
    pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
        let (mut temp,mut sum) = (x,0);
        while temp>0
        {
            sum+=temp%10;
            temp/=10;
        }
        if x%sum==0 { sum} else { -1}
    }
}
