impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let text = text.split_whitespace().collect::<Vec<_>>();
        let mut ans = vec![vec![0;26];text.len()];
        for i in 0..text.len()
        {
            text[i].as_bytes().into_iter().for_each(|x| ans[i][(x-97) as usize]+=1);
        }
        let mut sol = 0;
        for i in 0..text.len()
        {
            let mut temp = 0;
            broken_letters.as_bytes().into_iter().for_each(|x| if ans[i][(x-97)as usize]>0 { temp|=1;});
            if temp==1
            {   sol+=1;}
        }
        text.len() as i32-sol
    }
}
