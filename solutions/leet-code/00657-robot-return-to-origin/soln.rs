impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let (mut x,mut y)=(0,0);
        for i in moves.chars()
        {
            if i=='U'
            {   y+=1;}
            else if i=='D'
            {   y-=1;}
            else if i=='L'
            {   x-=1;}
            else if i=='R'
            {   x+=1;}
        
        }    
        x==0&&y==0
    }
}
