impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut ans = vec![0;26];
        s.chars().for_each(|x| ans[(x as u8 - 'a' as u8)as usize]+=1);
        let (mut l,mut r )= (0,i32::MAX);
        ans.into_iter().for_each(|x| { if x%2==1 { l = l.max(x);} else if x!=0{ r = r.min(x);}});
        l-r
    }
}
