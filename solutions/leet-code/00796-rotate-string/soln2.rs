impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len()!=goal.len(){ return false;}
        if s.is_empty() { return true;}
        (s.clone() +&s).contains(&goal)
    }
}
