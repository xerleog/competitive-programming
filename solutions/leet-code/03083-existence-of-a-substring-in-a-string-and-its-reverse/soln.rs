use std::collections::HashSet;
impl Solution {
    pub fn is_substring_present(s: String) -> bool {
        let forward = s.as_bytes().windows(2).collect::<HashSet<_>>();
        let s = s.bytes().rev().collect::<Vec<_>>();
        let backward = s.windows(2).collect::<HashSet<_>>();
        forward.intersection(&backward).count() > 0
    }
}
