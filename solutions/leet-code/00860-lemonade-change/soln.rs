impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut ans = vec![0;3];
        for i in  bills.into_iter()
        {
           match i
           {
            5 => ans[0]+=1,
            10 => if ans[0]>0 { ans[1]+=1; ans[0]-=1; } else { return false;}
            20 => if ans[0]>0&&ans[1]>0 {ans[0]-=1;ans[1]-=1} else if ans[0]>=3&&ans[1]==0 { ans[0]-=3;} else { return false;}
            _ => return false,
           }
        }
        true
    }
}
