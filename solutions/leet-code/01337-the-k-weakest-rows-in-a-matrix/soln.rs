impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let ans = mat.into_iter().map(|x| x.into_iter().sum::<i32>()).collect::<Vec<_>>();
        let mut temp = ans.into_iter().enumerate().map(|(x,y)| (y,x as i32)).collect::<Vec<_>>();
        temp.sort();
        temp.into_iter().map(|x| x.1).take(k as usize).collect::<Vec<_>>()
    }
}
