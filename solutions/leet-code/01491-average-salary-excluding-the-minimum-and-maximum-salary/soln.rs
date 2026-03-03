impl Solution {
    pub fn average(mut salary: Vec<i32>) -> f64 {
        salary.sort();
        let n = salary.len();
        salary[1..n-1].into_iter().map(|&x| x as f64).sum::<f64>()/(n-2) as f64
    }
}
