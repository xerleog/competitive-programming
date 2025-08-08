impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        box_types.sort_by(|a,b| b[1].cmp(&a[1]));
        let mut ans = 0;
        for i in box_types.into_iter()
        {
            if i[0]<truck_size
            {
                ans+=i[0]*i[1];
                truck_size-=i[0];
            }
            else
            {
                ans+=truck_size*i[1];
                break;
            }
        }
        ans
    }
}
