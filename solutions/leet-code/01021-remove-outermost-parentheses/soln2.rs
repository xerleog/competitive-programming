impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut res = String::with_capacity(s.len());
        let mut counter = 0;
        for c in s.chars() {
            let prev_counter = counter;
            counter += if c == '(' { 1 } else { -1 };
            if prev_counter | counter != 1 {
                res.push(c);
            }
        }
        res
    }
}
