impl Solution {
  pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
      let (c11, c12) = (coordinate1.as_bytes()[0], coordinate1.as_bytes()[1]);
      let (c21, c22) = (coordinate2.as_bytes()[0], coordinate2.as_bytes()[1]);
      (c11 % 2 == c21 % 2) == (c12 % 2 == c22 % 2)
  }
}
