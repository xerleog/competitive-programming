impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![];
      for i in queries
      {
        let (h,k,r,mut val)=(i[0],i[1],i[2],0);
        for j in &points
        {
            if ((j[0]-h)*(j[0]-h))+((j[1]-k)*(j[1]-k)) <= r*r
            {   val+=1;}
        }
        ans.push(val);
      } 
      ans 
    }
}
