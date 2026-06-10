impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut max_freq = i32::MIN;
        let mut counter = std::collections::BTreeMap::new();

        nums.iter().filter(|&&x| x % 2 == 0).for_each(|&x| {
            let freq = counter.entry(x).and_modify(|n| *n += 1).or_insert(1);
            if *freq > max_freq {
                max_freq = *freq;
            }
        });

        counter
            .into_iter()
            .find(|(_, freq)| *freq == max_freq)
            .map_or(-1, |(x, _)| x)
    }
}
