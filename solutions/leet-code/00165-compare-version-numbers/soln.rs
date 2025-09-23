impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        (0..version1.split('.').count().max(version2.split('.').count()))
        .map(|i| {
            let a = version1.split('.').nth(i).unwrap_or("0").parse::<i32>().unwrap_or(0);
            let b = version2.split('.').nth(i).unwrap_or("0").parse::<i32>().unwrap_or(0);
            match a.cmp(&b) {
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
                std::cmp::Ordering::Less => -1,
            }
        })
        .find(|&res| res != 0)
        .unwrap_or(0)
    }
}
