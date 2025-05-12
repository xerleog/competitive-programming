use std::collections::BTreeSet;
impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut ans = BTreeSet::new();
        let n = digits.len();
        for i in  0..n
        {
            for j in 0..n
            {
                for k in 0..n
                {
                    if i!=j && j!=k && k!=i
                    {
                        let temp = digits[i]*100+digits[j]*10+digits[k];
                        if temp>99 && temp%2==0
                        {   ans.insert(temp); }
                    }
                }
            }
        }
        ans.into_iter().collect()
    }
}
