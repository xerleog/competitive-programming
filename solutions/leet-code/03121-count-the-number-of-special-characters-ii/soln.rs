impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut ans = vec![vec![i32::MAX,-1];26];
        word.as_bytes().into_iter().enumerate().for_each(|(x,y)| if *y>96 { ans[(y-97) as usize][1] = (x as i32).max(ans[(y-97) as usize][1])} else { ans[(y-65) as usize][0] = (x as i32).min(ans[(y-65) as usize][0]) });
        ans.into_iter().filter(|x| x[0]>x[1] && x[0]!=i32::MAX && x[1]!= -1).count() as i32
    }
}
