impl Solution {
    pub fn reordered_power_of2(mut n: i32) -> bool {
        let mut ans = vec![];
        while n>0
        {
            ans.push(n%10);
            n/=10;
        }
        ans.sort();
        let (mut val,mut sol) = (1,false);
        for i in (1..32)
        {
            let mut temp = vec![];
            let mut cur = val;
            while cur>0
            {
                temp.push(cur%10);
                cur/=10;
            }
            temp.sort();
            if temp==ans
            {   sol|=true;}
            val*=2;
        }
        sol
    }
}
