impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let freq = words
            .iter()
            .map(|w| w.chars().fold(vec![0;26], |mut freq, c| { freq[(c as u8 - b'a') as usize] += 1; freq }))
            .collect::<Vec<_>>();

        Self::transpose(freq)
            .into_iter()
            .enumerate()
            .map(|(c, f)| (c, *f.iter().min().unwrap()))
            .map(|(c, f)| vec![((c as u8 + b'a') as char).to_string(); f as usize])
            .flatten()
            .collect()
    }

    fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> where T: Clone {
        (0..v[0].len())
            .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
            .collect()
    }
}
