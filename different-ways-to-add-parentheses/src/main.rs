struct Solution;

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let expression_str = expression.as_str();
        for i in 0..expression_str.len() {
            if &expression_str[i..i + 1] == "+"
                || &expression_str[i..i + 1] == "-"
                || &expression_str[i..i + 1] == "*"
            {
                let left_part = Self::diff_ways_to_compute(expression_str[0..i].to_string());
                let right_part = Self::diff_ways_to_compute(
                    expression_str[i + 1..expression_str.len()].to_string(),
                );
                for l in left_part {
                    for r in right_part.iter() {
                        result.push(Self::compute(l, *r, &expression_str[i..i + 1]))
                    }
                }
            }
        }
        if result.len() == 0 {
            result.push(expression.parse::<i32>().expect("Parse error"))
        }
        return result;
    }

    fn compute(left: i32, right: i32, operator: &str) -> i32 {
        println!("{}, {}", left, right);
        match operator {
            "+" => return left + right,
            "-" => return left - right,
            "*" => return left * right,
            _ => 0,
        }
    }
}

fn main() {
    let a = Solution::diff_ways_to_compute("2*3-4*5".to_string());
    println!("{:?}", a);
}
