impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
       let sol = vec![
        vec!['a', 'b', 'c'],       
        vec!['d', 'e', 'f'],       
        vec!['g', 'h', 'i'],       
        vec!['j', 'k', 'l'],       
        vec!['m', 'n', 'o'],       
        vec!['p', 'q', 'r', 's'],
        vec!['t', 'u', 'v'],     
        vec!['w', 'x', 'y', 'z']
    ];
    let mut ans = vec![];
    for i in digits.as_bytes().into_iter()
    {
        let mut temp = vec![];
        if ans.len()==0 { temp = sol[(i-50) as usize].clone().into_iter().map(|x| x.to_string()).collect::<Vec<_>>();}
        for j in &ans
        {
            for k in &sol[(i-50) as usize]
            {
                temp.push(format!("{}{}",j,k));
            }
        }
        ans.clear();
        ans = temp;
    }
    ans
    }
}
