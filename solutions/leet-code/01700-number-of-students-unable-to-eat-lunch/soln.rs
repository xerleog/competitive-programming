impl Solution {
    pub fn count_students(mut students: Vec<i32>, mut sandwiches: Vec<i32>) -> i32 {
        while students.len() > 0 && students.contains(&sandwiches[0]) {
            students.remove(students.iter().position(|&e| e == sandwiches[0]).unwrap());
            sandwiches.remove(0);
        }
        students.len() as i32
    }
}
