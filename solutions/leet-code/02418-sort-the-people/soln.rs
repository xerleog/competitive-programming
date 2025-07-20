impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut ans = heights.into_iter().zip(names.into_iter()).collect::<Vec<_>>();
        ans.sort_by(|(a,_),(b,_)| b.cmp(&a));
        ans.into_iter().map(|(_,x)| x).collect::<Vec<_>>()
    }
}
