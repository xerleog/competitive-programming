impl Solution {
    pub fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
        let (mut ans1,mut ans2,n) = (vec![],vec![],nums.len());
        nums.into_iter().enumerate().for_each(|x| if x.0%2==0 {ans1.push(x.1);} else {ans2.push(x.1);});
        ans1.sort();
        ans2.sort_by(|a, b| b.cmp(a));
        (0..n).into_iter().map(|i| if i%2==0 { ans1[i/2]} else { ans2[i/2]}).collect()
    }
}
