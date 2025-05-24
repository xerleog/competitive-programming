impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        for x in 1..arr.len()-1
        {
            if arr[x]>arr[x-1] && arr[x]>arr[x+1]
            {   return x as i32;}
        }
        0
    }
}
