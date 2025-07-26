impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut ans = vec![];
        for i in operations
        {
            if i=="+"
            {
                let a = ans.pop().unwrap();
                let b = ans.pop().unwrap();
                ans.extend(vec![b,a,a+b]);                
            }
            else if i== "D"
            {
                let a = ans.pop().unwrap();
                ans.extend(vec![a,a*2]);
            } 
            else if i == "C"
            {
                ans.pop();
            }
            else
            {
                ans.push(i.parse::<i32>().unwrap());
            }
        }
        ans.into_iter().sum::<i32>()
    }
}
