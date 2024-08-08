fn minimum_pushes(word: String) -> i32 {
    let mut count = vec![0; 26];
    for w in word.chars() {
        count[(w as usize) - ('a' as usize)] += 1;
    }
    count.sort_by(|a, b| b.cmp(a));
    let mut res = 0;
    for (i, freq) in count.iter().enumerate() {
        if *freq == 0 {
            break;
        }
        res += (i / 8 + 1) * freq;
    }
    res as i32
}

fn main() {
    let a = "abcde".to_string();
    let ex = minimum_pushes(a);
    println!("{}", ex);
}
