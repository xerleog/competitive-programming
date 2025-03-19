impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let mut ans = vec![0;26];
        s.chars().enumerate().for_each(|(x,y)| {ans[(y as u8-'a' as u8) as usize]+=x as i32;});
        t.chars().enumerate().for_each(|(x,y)| {let temp = (y as u8-'a' as u8) as usize; ans[temp]=(ans[temp]-x as i32).abs();});
        ans.iter().sum::<i32>()
    }
}
