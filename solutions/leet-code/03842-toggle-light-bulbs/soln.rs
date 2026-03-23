impl Solution {
    pub fn toggle_light_bulbs(bulbs: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0;101];
        bulbs.into_iter().for_each(|x| ans[x as usize]+=1);
        ans.into_iter().enumerate().filter_map(|x| if x.1!=0 && x.1%2==1 { Some(x.0 as i32)} else { None}).collect()
    }
}
