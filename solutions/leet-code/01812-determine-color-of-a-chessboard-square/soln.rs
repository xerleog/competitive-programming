impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let val = coordinates.as_bytes().into_iter().collect::<Vec<_>>();
        ((val[0]-97)^(val[1]-48))&1==0
    }
}
