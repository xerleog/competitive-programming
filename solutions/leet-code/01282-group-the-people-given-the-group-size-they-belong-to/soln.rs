impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![];501];
        let mut sol =vec![];
        group_sizes.into_iter().enumerate().for_each(|(a,b)| ans[b as usize].push(a as i32));
        for i in 1..501
        {
            ans[i].chunks(i).for_each(|x| sol.push(x.to_vec()));
        }   
        sol
    }
}
