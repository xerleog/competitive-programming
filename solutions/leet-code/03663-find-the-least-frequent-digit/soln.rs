impl Solution {
    pub fn get_least_frequent_digit(mut n: i32) -> i32 {
        let mut ans = vec![0;10];
        while n>0
        {
            ans[(n%10)as usize]+=1;
            n/=10;
        }
        let mi = ans.clone().into_iter().filter(|&x| x!=0).min().unwrap();
        ans.into_iter().position(|x| x==mi).unwrap() as i32
    }
}
