impl Solution {
    pub fn last_stone_weight(mut stones: Vec<i32>) -> i32 {
        while stones.len()>1
        {
            stones.sort();
            let (a,b)=(stones.pop().unwrap(),stones.pop().unwrap());
            if a != b
            { stones.push((a-b).abs());}
        }
        stones.pop().unwrap_or(0)
    }
}
