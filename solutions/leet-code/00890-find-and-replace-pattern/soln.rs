impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        words
            .into_iter()
            .filter(|w| {
                let mut pmap = vec!['\0'; 26];
                w.chars()
                    .zip(pattern.chars().map(|p| p as usize - 'a' as usize))
                    .all(|(c, i)| {
                        if pmap[i] == '\0' {
                            pmap[i] = c;
                            pmap.iter().filter(|&&c| c == pmap[i]).count() == 1
                        } else {
                            c == pmap[i]
                        }
                    })
            })
            .collect()
    }
}
