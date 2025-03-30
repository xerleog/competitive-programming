impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0;100];
        nums.iter().for_each(|&x| ans[x as usize]+=1);
        ans.into_iter().enumerate().filter(|(a,b)| *b==2).map(|(x,y)| x as i32).collect::<Vec<_>>()
    }
}
