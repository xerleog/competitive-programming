use std::iter::repeat;
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut ans = vec![0;3];
        let mut temp = Vec::new();
        nums.into_iter().for_each(|x| ans[*x as usize]+=1 );
        temp.extend(repeat(0).take(ans[0]));
        temp.extend(repeat(1).take(ans[1]));
        temp.extend(repeat(2).take(ans[2]));
        *nums = temp;
    }
}
