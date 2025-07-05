impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut ans = vec![0;501];
        arr.into_iter().for_each(|x| ans[x as usize]+=1);
        for (i,j) in ans.into_iter().enumerate().rev()
        {
            if i as i32==j && i!=0
            {   return j;}
        }
        return -1;
    }
}
