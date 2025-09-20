impl Solution {
    pub fn digit_count(num: String) -> bool {
        let mut ans = vec![0;10];
        num.as_bytes().into_iter().for_each(|x| ans[(x-48)as usize]+=1);
        format!("{}{}",num,"0".repeat(10-num.len()))== ans.into_iter().map(|x| x.to_string()).collect::<String>()
    }
}
