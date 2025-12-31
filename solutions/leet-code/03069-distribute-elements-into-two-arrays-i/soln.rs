impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let (mut ans1,mut ans2)=(vec![nums[0]],vec![nums[1]]);
        for i in nums.into_iter().skip(2)
        {
            if ans1.last().unwrap()>ans2.last().unwrap(){ans1.push(i);}
            else { ans2.push(i);}
        }
        ans1.extend(ans2);
        ans1
    }
}
