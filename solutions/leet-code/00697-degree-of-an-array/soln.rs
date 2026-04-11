impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans = vec![vec![];50000];
        nums.into_iter().enumerate().for_each(|(x,y)| ans[y as usize].push(x));
        ans.sort_by(|a,b| b.len().cmp(&a.len()));
        let (mut sol,mut i)=((ans[0].last().unwrap()-ans[0][0]+1)as i32,1);
        while ans[i].len()==ans[0].len()
        {
            sol = sol.min((ans[i].last().unwrap()-ans[i][0]+1)as i32);
            i+=1;
        }
        sol
    }
}
