impl Solution {
    pub fn reformat_date(date: String) -> String {
        let year = vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
        let mut ans = date.split(" ").collect::<Vec<_>>();
        ans.reverse();
        ans[2] = &ans[2][..ans[2].len()-2];
        let temp = (year.iter().position(|&m| m == ans[1]).unwrap()+1);
        let temp2 = if temp < 10 { format!("0{}",temp)} else {  format!("{}",temp)};
        ans[1]= &temp2;
        let temp3 = if ans[2].len()==1 { format!("0{}",ans[2])} else {  format!("{}",ans[2])};
        ans[2] = &temp3;
        ans.join("-")
    }
}
