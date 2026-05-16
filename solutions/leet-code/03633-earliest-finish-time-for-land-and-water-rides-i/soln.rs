use std::cmp::min;

impl Solution {
    pub fn earliest_finish_time(land_start_time: Vec<i32>, land_duration: Vec<i32>, water_start_time: Vec<i32>, water_duration: Vec<i32>) -> i32 {
        let len_l = land_duration.len();
        let len_w = water_duration.len();
        let mut mn = i32::MAX;
        for l in 0..len_l {
            let ls = land_start_time[l];
            let ld = land_duration[l];
            for w in 0..len_w {
                let ws = water_start_time[w];
                let wd = water_duration[w];
                if ws < (ls + ld) {
                    mn = min(mn, ls + ld + wd);
                } else {
                    mn = min(mn, ws + wd)
                }
            }
        }
        for w in 0..len_w {
            let ws = water_start_time[w];
            let wd = water_duration[w];
            for l in 0..len_l {
                let ls = land_start_time[l];
                let ld = land_duration[l];
                if ls < (ws + wd) {
                    mn = min(mn, ws + wd + ld);
                } else {
                    mn = min(mn, ls + ld);
                }
            }
        }
        mn
    }
}
