impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let (mut ans, k) = (s.chars().collect::<Vec<_>>(),k as usize);
        if ans.len()%k!=0
        {
            for _ in 0..k-(ans.len()%k)
            {   ans.push(fill);}
        }
        ans.chunks(k).map(|x| {x.into_iter().collect::<String>()}).collect()

    }
}
