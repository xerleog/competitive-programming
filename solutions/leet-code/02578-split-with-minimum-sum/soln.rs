impl Solution {
    pub fn split_num(mut num: i32) -> i32 {
        let mut ans = vec![];
        while num>0
        {
            ans.push(num%10);
            num/=10;
        }
        ans.sort();
        let (n1,n2) = ans.iter().enumerate().fold((0, 0), |(n1, n2), (i, &d)| {
            if i % 2 == 0 { (n1 * 10 + d, n2) } else { (n1, n2 * 10 + d) }  
        });
        n1+n2
    }
}
