impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
      for i in  0..nums.len()
      {
            let (mut temp, mut val) =(nums[i],0);
            while temp>0
            {
                val+=temp%10;
                temp/=10;
            }
            if val==i as i32
            {   return i as i32;}
      }  
      -1
    }
}
