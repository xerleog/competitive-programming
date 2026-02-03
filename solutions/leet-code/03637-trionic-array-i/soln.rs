impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        let length = nums.len();
        if length < 4 || nums[0] > nums[1] {
            return false;
        }
        let mut ind = 0;
        while nums[ind] < nums[ind + 1] {
            ind += 1;
            if ind == length - 1 {
                return false;
            }
        }
        while nums[ind] > nums[ind + 1] {
            ind += 1;
            if ind == length - 1 {
                return false;
            }
        }
        while nums[ind] < nums[ind + 1] {
            ind += 1;
            if ind == length - 1 {
                return true;
            }
        }
        false
    }
}
