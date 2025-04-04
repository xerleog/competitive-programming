impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let ma = candies.clone().into_iter().max().unwrap();
        candies.into_iter().map(|x| x+extra_candies>=ma).collect::<Vec<_>>()
    }
}
