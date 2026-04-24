impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let (l,r,e) = moves.chars().fold((0,0,0), |(l,r,e), c| {
            if c == 'L' {
                (l+1,r,e)
            } else if c == 'R' {
                (l,r+1,e)
            } else {
                (l,r,e+1)
            }
        });
        l.max(r) - l.min(r) + e
    }
}
