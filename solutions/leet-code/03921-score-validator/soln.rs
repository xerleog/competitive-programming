impl Solution {
    pub fn score_validator(events: Vec<String>) -> Vec<i32> {
        let (mut ans,mut sol)=(0,0);
        for i in events
        {
            match i.as_str() {
                "0"| "1"| "2"| "3"| "4"| "6" => ans+= i.parse::<i32>().unwrap(),
                "W" => sol +=1,
                "WD"|"NB" => ans+=1,
                _ => {}
            }
            if sol == 10 { break;}
        }
        vec![ans,sol]
    }
}
