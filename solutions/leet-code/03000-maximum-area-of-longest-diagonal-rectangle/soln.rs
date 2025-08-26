impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let mut ans = dimensions.into_iter().map(|x| (((x[0]*x[0]+x[1]*x[1])as f64).sqrt(),x[0]*x[1])).collect::<Vec<_>>();
        let (mut val,mut area)=(0.0,0);
        for (i,j) in ans
        {
            if i>val
            {
                val=i;
                area=j;
            }
            else if i==val
            {
                area=area.max(j);
            }
        }
        area
    }
}
