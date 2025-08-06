impl Solution {
    pub fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in nums
        {
            let (mut temp,mut n) =(0,i);
            while n>0
            {
                temp = temp.max(n%10);
                n/=10;
            }
            if i<10
            {
                ans+=temp;
            }
            else if i<100
            {
                ans+=temp*11;
            }
            else if i<1000
            {
                ans+=temp*111;
            }
            else
            {
                ans+=1111;
            }
        }
        ans
    }
}
