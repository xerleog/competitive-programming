impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        points.windows(2).map(|x| { let l1 =  (x[1][0]-x[0][0]).abs(); let l2 = (x[1][1]-x[0][1]).abs(); l1.min(l2)+(l2-l1).abs()}).sum::<i32>()
    }
}
