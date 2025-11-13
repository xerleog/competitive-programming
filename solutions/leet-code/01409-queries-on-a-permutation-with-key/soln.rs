impl Solution {
    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        let mut ans = vec![];
        let mut pem = (1..=m).collect::<Vec<i32>>();
        for i in queries
        {
            let temp = pem.iter().position(|&x| x==i).unwrap();
            ans.push(temp as i32);
            pem[..=temp].rotate_right(1);
        }
        ans
    }
}
