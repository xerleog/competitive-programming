impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut ans = vec![0;26];
        s.clone().as_bytes().into_iter().for_each(|x| ans[(x-97) as usize]+=1);
        for (i,j) in s.as_bytes().into_iter().enumerate()
        {
            if ans[(j-97)as usize]==1 { return i as i32;}
        }
        return -1;
    }
}
