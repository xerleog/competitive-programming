impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let ans = vec!["type", "color", "name"];
        let y = ans.iter().position(|&x| x==rule_key).unwrap();
        println!("{:?}",y);
        (0..items.len()).filter(|&x| items[x][y]==rule_value).count() as i32
        
    }
}
