impl Solution {
    pub fn minimum_index(capacity: Vec<i32>, item_size: i32) -> i32 {
        let (mut i,mut abx)=(-1,i32::MAX);
        (0..capacity.len()).into_iter().for_each(|x| {let val = capacity[x]-item_size; if (val >=0 && val<abx) { abx = val; i=x as i32; }});
        i
    }
}
