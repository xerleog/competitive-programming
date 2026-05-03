impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        let mut s = s.chars().collect::<Vec<_>>();
        let goal = goal.chars().collect::<Vec<_>>();
        for i in 0..s.len()
        {
            s.rotate_right(1);
            if s==goal
            {   return true;}
        }
        false

    }
}
