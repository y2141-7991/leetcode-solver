fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
    let mut min_num = arrays[0][0];
    let mut max_num = arrays[0][arrays[0].len() - 1];
    let mut max_dis = 0;

    for i in 1..arrays.len() - 1 {
        min_num = vec![min_num, arrays[i][0]].into_iter().min().unwrap();
        max_num = vec![max_num, arrays[i][arrays[i].len() - 1]]
            .into_iter()
            .max()
            .unwrap();

        let mut max_dis1 = vec![
            (min_num - arrays[i + 1][arrays[i + 1].len() - 1]).abs(),
            (max_num - arrays[i + 1][0]).abs(),
        ]
        .into_iter()
        .max()
        .unwrap();
        max_dis = vec![max_dis, max_dis1].into_iter().max().unwrap();
    }

    max_dis
}

fn main() {
    let a: Vec<Vec<i32>> = vec![
        [-1, 1].to_vec(),
        [-3, 1, 4].to_vec(),
        [-2, -1, 0, 2].to_vec(),
    ];
    let x = max_distance(a);
    println!("{}", x);
}
