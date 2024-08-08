static MY_DATA: &'static [(i32, &str)] = &[
    (1000000000, "Billion"),
    (1000000, "Million"),
    (1000, "Thousand"),
    (100, "Hundred"),
    (90, "Ninety"),
    (80, "Eighty"),
    (70, "Seventy"),
    (60, "Sixty"),
    (50, "Fifty"),
    (40, "Forty"),
    (30, "Thirty"),
    (20, "Twenty"),
    (19, "Nineteen"),
    (18, "Eighteen"),
    (17, "Seventeen"),
    (16, "Sixteen"),
    (15, "Fifteen"),
    (14, "Fourteen"),
    (13, "Thirteen"),
    (12, "Twelve"),
    (11, "Eleven"),
    (10, "Ten"),
    (9, "Nine"),
    (8, "Eight"),
    (7, "Seven"),
    (6, "Six"),
    (5, "Five"),
    (4, "Four"),
    (3, "Three"),
    (2, "Two"),
    (1, "One"),
    (0, "Zero"),
];

fn create_list_word_num(num: i32, num_vec: &mut Vec<i32>) -> Vec<i32> {
    let vec_num_word: Vec<(i32, &str)> = MY_DATA.to_vec();

    if num == 0 {
        let mut rv: Vec<i32> = Vec::new();
        rv.push(0);
        return rv;
    }

    if num > vec_num_word[0].0 {
        let (m, n) = divmod(num, vec_num_word[0].0);
        // println!("{:?}, {}, {}", num_vec, num, n);
        num_vec.extend(vec![m, vec_num_word[0].0]);
        let tv: Vec<i32> = create_list_word_num(n, &mut Vec::new());
        num_vec.extend(tv)
    }

    for i in 0..vec_num_word.len() {
        if num == vec_num_word[i].0 {
            let (m, _) = divmod(num, vec_num_word[i].0);
            if num >= 100 {
                num_vec.extend(vec![m, num])
            } else {
                num_vec.push(num)
            }
        }

        if vec_num_word[i].0 > num && num > vec_num_word[i + 1].0 && num > 0 {
            let (m, n) = divmod(num, vec_num_word[i + 1].0);
            if num >= 100 {
                if m >= 10 {
                    num_vec.extend(create_list_word_num(m, &mut Vec::new()));
                    num_vec.push(vec_num_word[i + 1].0)
                } else {
                    num_vec.extend(vec![m, vec_num_word[i + 1].0]);
                }
            } else {
                num_vec.push(vec_num_word[i + 1].0);
            }
            create_list_word_num(n, num_vec);
        }
    }
    return num_vec.to_vec();
}

fn divmod(x: i32, y: i32) -> (i32, i32) {
    (x / y, x % y)
}

pub fn number_to_words(num: i32) -> String {
    let mut string_num = String::new();
    let vec_num_word: Vec<(i32, &str)> = MY_DATA.to_vec();
    let num = create_list_word_num(num, &mut Vec::new());
    for m in num {
        for (k, v) in vec_num_word.clone().into_iter() {
            if m == k {
                let string = format!("{} ", v);
                string_num.push_str(&string)
            }
        }
    }
    string_num.trim().to_string()
}

fn main() {
    println!("{}", number_to_words(1000000000))
}
