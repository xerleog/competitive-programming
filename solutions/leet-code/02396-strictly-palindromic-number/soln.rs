impl Solution {
    pub fn is_strictly_palindromic(n: i32) -> bool {
        for i in 2..=n-2
        {
            let mut temp = n.clone();
            let mut val = String::new();
            while temp>0
            {
                let r = temp % i;
                temp /= i;
                val.push_str(&format!("{}", r));
            }
            if !(val==val.chars().rev().collect::<String>())
            {
                return false;
            }
        }
        true
    }
}
