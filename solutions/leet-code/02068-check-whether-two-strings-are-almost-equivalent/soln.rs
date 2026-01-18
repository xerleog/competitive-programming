impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let (mut f1,mut f2)=(vec![0;26],vec![0;26]);
        word1.as_bytes().into_iter().for_each(|x| f1[(x-97)as usize]+=1);
        word2.as_bytes().into_iter().for_each(|x| f2[(x-97)as usize]+=1);
        f1.iter().zip(f2.iter()) .all(|(n1, n2)| ((n1 - n2) as i32).abs() <= 3)
    }
}
