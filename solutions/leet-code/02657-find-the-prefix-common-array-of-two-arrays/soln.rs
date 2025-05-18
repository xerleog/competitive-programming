impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0;a.len()];
        let mut sol = vec![];
        for i in 0..a.len()
        {
            ans[(a[i]-1)as usize]+=1;
            ans[(b[i]-1)as usize]+=1;
            sol.push(ans.clone().into_iter().filter(|x| x%2==0&& *x!=0).count() as i32);
        }
        sol
    }
}
