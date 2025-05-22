use std::collections::HashMap;
impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let _len_k: usize = key.len();
        let _len_m: usize = message.len();
        const SPACE: char = ' ';
        let dict: HashMap<char, char> = {
            let mut map: HashMap<char, char> = HashMap::with_capacity(26);
            let mut iter_all_letters = 'a'..='z';
            for ch in key.chars().filter(|&c| c != SPACE) {
                map.entry(ch)
                    .or_insert_with(|| iter_all_letters.next().unwrap());
            }
            map
        };
        message
            .chars()
            .map(|c| *dict.get(&c).unwrap_or(&c))
            .collect()
    }
}
