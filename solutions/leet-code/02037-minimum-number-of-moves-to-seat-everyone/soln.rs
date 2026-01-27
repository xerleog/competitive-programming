impl Solution {
    pub fn min_moves_to_seat(mut seats: Vec<i32>,mut students: Vec<i32>) -> i32 {
        seats.sort();
        students.sort();
        seats.into_iter().zip(students.into_iter()).map(|x| (x.0-x.1).abs()).sum()
    }
}
