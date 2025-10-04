impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut i,mut j, mut ans)=(0,height.len()-1,0);
        while i<j
        {
            ans = ans.max(height[i].min(height[j])*(j-i) as i32);
            if height[i]< height[j] { i+=1;} else { j-=1;}
        }
        ans
    }
}
