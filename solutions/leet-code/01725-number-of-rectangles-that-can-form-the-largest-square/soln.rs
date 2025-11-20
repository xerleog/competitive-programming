use std::collections::HashMap;
impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let ans = rectangles.into_iter().map(|x| x[0].min(x[1])).collect::<Vec<_>>();
          ans.iter()
            .fold(HashMap::new(), |mut acc, &x| { *acc.entry(x).or_insert(0) += 1; acc })
            .into_iter()
            .max_by_key(|&(val,_)| val)
            .unwrap()
            .1
            
    }
}
