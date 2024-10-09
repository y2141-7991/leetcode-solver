struct Solution;

impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let mut a = 0;
        let mut list_skill: Vec<i64> = skill.clone().iter().map(|n| *n as i64).collect();

        list_skill.sort();

        let total_num = list_skill[0] + list_skill[skill.len() - 1];

        for n in 0..(skill.len() / 2) {
            if list_skill[n] + list_skill[skill.len() - 1 - n] != total_num {
                return -1;
            }
            a += list_skill[n] * list_skill[skill.len() - 1 - n]
        }
        a
    }
}

fn main() {
    let a = vec![3, 2, 5, 1, 3, 4];

    println!("{}", Solution::divide_players(a));
}
