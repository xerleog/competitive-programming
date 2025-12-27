impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let (mut ans,mut i) = (vec![],0);
        while ans.len()<arr.len()
        {
            if arr[i]!=0 { ans.push(arr[i]);}
            else { ans.extend(vec![0,0]);}
            i+=1;
        }
        *arr = ans.into_iter().take(arr.len()).collect();
    }
}
