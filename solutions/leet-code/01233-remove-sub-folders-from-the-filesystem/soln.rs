impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        folder.sort();
        let mut ans = vec![];
        ans.push(folder[0].clone());
        for i in 1..folder.len()
        {
            let mut last = ans[ans.len()-1].clone();
            last+="/";
            if !folder[i].starts_with(&last)
            {   ans.push(folder[i].clone());}
        }
        ans
    }
}
