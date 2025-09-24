impl Solution {
    pub fn concat_hex36(n: i32) -> String {
        let (mut ans,mut sol) = (n*n*n,String::new());
        while ans>0
        {
            let digit = ans % 36;
            let c = if digit < 10 { (b'0' + digit as u8) as char } else { (b'A' + (digit - 10) as u8) as char };
            sol.insert(0, c);
            ans /= 36;
        } 
        format!("{:X}{}", n*n,sol)
    }
}
