impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        grid.into_iter().enumerate().map(|(a,b)| (b.into_iter().filter(|&x| x==1).count(),a)).max().unwrap().1 as i32
    }
}
