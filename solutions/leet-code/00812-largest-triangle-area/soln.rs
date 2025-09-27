impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut ans = 0_f64;
        for i in 0..points.len()-2
        {
            for j in i+1..points.len()-1
            {
                for k in j+1..points.len()
                {
                    let (x1,y1) = (points[i][0],points[i][1]);
                    let (x2,y2) = (points[j][0],points[j][1]);
                    let (x3,y3) = (points[k][0],points[k][1]);
                    // Shoelace's Formula
                    let area = 0.5 * (x1*(y2-y3)+x2*(y3-y1)+x3*(y1-y2)).abs() as f64;
                    ans = ans.max(area);
                }
            }
        }
        ans
    }
}
