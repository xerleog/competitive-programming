impl Solution {
    pub fn decode(encoded: Vec<i32>, mut first: i32) -> Vec<i32> {
        encoded.into_iter().fold(vec![first], |mut acc, i| {
			acc.push(acc[acc.len() - 1] ^ i);
			acc
		})
    }
}
