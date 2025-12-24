impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        let mut sum = apple.into_iter().sum::<i32>();
        capacity.sort();
        let mut ans = 0;
        while sum>0
        {
            ans+=1;
            sum-=capacity.pop().unwrap();
        }
        ans
    }
}
