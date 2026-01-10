impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let (mut i,mut j) = (0,0);
        for temp in commands.into_iter()
        {
            match temp.as_str()
            {
                "UP" => i-=1,
                "RIGHT" => j+=1,
                "DOWN" => i+=1,
                "LEFT" => j-=1,
                _ => ()
            }
        }   
        i*n+j
    }
}
