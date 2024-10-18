struct Solution;

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        // for a in 0..num.clone().to_string().len() {
        //     let mut num_str = num.to_string();
        //     let b = num_str.remove(a);

        //     println!("{:?}", num_str);
        //     println!("{:?}", b);
        // }
        let num_str = num.to_string();

        let mut p1 = 0;
        let mut p2 = 0;
        let mut max_i = num_str.len() - 1;
        for i in (0..num_str.len()).rev() {
            println!("{:?}, {:?}", &num_str.chars().nth(max_i), &num_str.chars().nth(i));
            if &num_str.chars().nth(max_i) < &num_str.chars().nth(i) {
                max_i = i;
            }
            else if &num_str.chars().nth(max_i) > &num_str.chars().nth(i) {
                p1 = max_i;
                p2 = i;
            }
        }
        let mut a:Vec<_> = num_str.chars().collect();
        a.swap(p1, p2);
        let b = a.into_iter().collect::<String>();
        b.parse::<i32>().unwrap()
    }
}

fn main() {
    let a = 9123;
    println!("{}", Solution::maximum_swap(a));
}
