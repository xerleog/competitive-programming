impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
        nums.sort_by(|a,b| format!("{}{}",b,a).cmp(&(format!("{}{}",a,b))));
        if nums[0]=="0" { return "0".to_string();}
        nums.join("")
    }
}
