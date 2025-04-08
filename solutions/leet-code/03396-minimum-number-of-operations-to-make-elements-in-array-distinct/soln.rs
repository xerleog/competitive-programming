impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut ans = vec![0;100];
        let mut sol=0;
        nums.clone().into_iter().for_each(|x| ans[(x-1)as usize]+=1);
        for i in nums.chunks(3)
        {
            if ans.clone().into_iter().all(|x| x==1||x==0)
            {   return sol;}
            else
            {
                for j in i
                {
                    ans[(j-1) as usize]-=1;
                }
                sol+=1;
            }
        }
        sol
    }
}
