impl Solution {
    pub fn number_of_alternating_groups(mut colors: Vec<i32>) -> i32 {
        let mut colours = colors.clone();
        colours.extend(&colors[..2]);
        colours.windows(3).filter(|x| x[0]==x[2]&& x[0]!=x[1]).count() as i32
    }
}
