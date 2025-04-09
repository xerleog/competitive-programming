impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = vec![0;101];
        nums.into_iter().for_each(|x| ans[x as usize]+=1);
        if ans[0..k as usize].iter().all(|x| *x==0)
        {   ans[(k+1) as usize..].into_iter().filter(|&x| *x>0).count() as i32}
        else
        {   return -1;}
    }
}
