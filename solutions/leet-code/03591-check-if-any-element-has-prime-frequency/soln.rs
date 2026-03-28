impl Solution {
    pub fn check_prime_frequency(nums: Vec<i32>) -> bool {
        let primes_arr = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
        let mut ans = vec![0;101];
        nums.into_iter().for_each(|x| ans[x as usize]+=1);
        ans.into_iter().any(|x| primes_arr.contains(&x))
    }
}
