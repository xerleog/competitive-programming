impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let (mut left,mut right,mut ans) =(0,nums.iter().sum::<i32>(),0);
        nums.clone().into_iter().take(nums.len()-1).for_each(|x| {left+=x;right-=x; if (left-right)%2==0 { ans+=1;}});
        ans
    }
}
