impl Solution {
    pub fn make_good(s: String) -> String {
        let mut stack: Vec<char> = Vec::new();
        
        for l in s.chars() {
            if stack.len() > 0 && (l as i32 - stack[stack.len() - 1] as i32).abs() == 32 {
                stack.pop();
            }
            else {
                stack.push(l);
            }
        }
        
        return stack.iter().collect();
    }
}
