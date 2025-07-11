impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let mut ans = vec![0;51];
        nums.into_iter().for_each(|x| ans[x as usize]+=1);
        let val = (0..51).into_iter().filter(|&x| ans[x]==2).collect::<Vec<_>>();
        val.into_iter().fold(0,|a,c| a^c as i32)
    }
}
