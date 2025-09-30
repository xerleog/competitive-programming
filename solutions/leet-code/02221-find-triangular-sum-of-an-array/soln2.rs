impl Solution
{
    pub fn triangular_sum(nums: Vec<i32>) -> i32
    {
        let mut sum_array = nums;

        while sum_array.len() > 1
        {
            for i in 0..sum_array.len() - 1
            {
                sum_array[i] = (sum_array[i] + sum_array[i + 1]) % 10;
            }
            sum_array.pop();
        }

        sum_array[0]
    }
}
