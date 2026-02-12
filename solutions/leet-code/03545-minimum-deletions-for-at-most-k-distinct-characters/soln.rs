impl Solution {
    pub fn min_deletion(s: String, k: i32) -> i32 {
        let mut ans = vec![0;26];
        s.as_bytes().into_iter().for_each(|x| ans[(x-97)as usize]+=1);
        ans.sort_unstable();
        ans[0..26-k as usize].into_iter().sum()
    }
}
