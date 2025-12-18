impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut ans = vec![0;26];
        let mut sol = "".to_string();
        s.as_bytes().into_iter().for_each(|x| ans[(x-97) as usize]+=1);
        while ans.clone().into_iter().sum::<i32>()!=0
        {
            (0..26).into_iter().for_each(|x| if ans[x]>0 { sol.push((97+x as u8) as char);ans[x]-=1;});
            (0..26).into_iter().rev().for_each(|x| if ans[x]>0 { sol.push((97+x as u8) as char);ans[x]-=1;});
        }
        sol
    }
}
