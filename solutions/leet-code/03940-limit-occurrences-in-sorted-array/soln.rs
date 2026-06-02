impl Solution {
    pub fn limit_occurrences(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ans = vec![];
        let (mut cval,mut cnt)=(0,0);
        for i in nums.into_iter()
        {
            if cval != i
            {
                cval=i; cnt =1;
                ans.push(cval);
            }
            else if cval == i && cnt <k{ cnt+=1; ans.push(cval);}
        }
        ans
    }
}
