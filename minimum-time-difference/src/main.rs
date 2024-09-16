use std::i32;

struct Solution;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut minutes: Vec<i32> = Vec::new();
        for time_point in time_points {
            let b = time_point.split(':');
            let collections = b.collect::<Vec<&str>>();
            minutes.push(Self::convert_to_minute(collections));
        }

        minutes.sort_by(|a, b| a.cmp(b));
        println!("{:?}", minutes);
        let mut min_diff: i32 = i32::MAX;
        for i in 0..minutes.len() - 1 {
            min_diff = *vec![min_diff, minutes[i+1] - minutes[i]].iter().min().unwrap();
        }
        min_diff = *vec![min_diff, 1440 + minutes[0] - minutes[minutes.len()-1]].iter().min().unwrap();
        min_diff
    }

    fn convert_to_minute(point: Vec<&str>) -> i32 {
        let a: Vec<i32> = point.iter().map(|b| b.parse::<i32>().unwrap()).collect();
        let hour_minutes = a[0] * 60;
        let sum_minutes = hour_minutes + a[1];
        sum_minutes
    }
}

fn main() {
    let a = Solution::find_min_difference(vec!["12:12".to_string(), "00:13".to_string()]);
    println!("{}", a)
}
