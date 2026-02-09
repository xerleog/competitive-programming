impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let (mut ans,mut sol) = (vec![0;26],vec![0;26]);
        s.as_bytes().into_iter().for_each(|x| ans[(x-97)as usize]+=1);   
        target.as_bytes().into_iter().for_each(|x| sol[(x-97)as usize]+=1);
        ans.into_iter().zip(sol.into_iter()).flat_map(|(a,b)| (b>0).then(|| a/b)).min().unwrap()
    }
}
