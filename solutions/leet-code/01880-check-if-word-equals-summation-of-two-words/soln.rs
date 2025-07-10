impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        let v = [first_word, second_word, target_word].map(|x| {
            x.bytes()
                .map(|x| (x - b'a').to_string())
                .collect::<String>()
                .parse::<i32>()
                .unwrap()
        });
        v[0] + v[1] == v[2]
    }
}
