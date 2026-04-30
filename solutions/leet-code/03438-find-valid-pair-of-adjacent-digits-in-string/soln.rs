impl Solution {
    pub fn find_valid_pair(s: String) -> String {
        let mut ans = vec![0;10];
        s.clone().as_bytes().into_iter().for_each(|x| ans[(x-48) as usize]+=1);
        s.as_bytes().windows(2).find(|x| ans[(x[0] - 48) as usize] == x[0] - 48 && ans[(x[1] - 48) as usize] == x[1] - 48 && x[0] != x[1]).map(|y| String::from_utf8(y.to_vec()).unwrap()).unwrap_or("".to_string())
    }
}
