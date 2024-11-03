struct Solution;

impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let mut folder = folder;
        folder.sort();
        let mut sub_folders: Vec<String> = Vec::new();
        sub_folders.push(folder[0].clone());
        for i in 1..folder.len() {
            let mut last = sub_folders[sub_folders.len() - 1].clone();
            last += "/";
            if !folder[i].starts_with(&last) {
                sub_folders.push(folder[i].clone());
            }
        }

        sub_folders
    }
}

fn main() {
    let a = vec![
        "/a".to_string(),
        "/a/b".to_string(),
        "/c/d".to_string(),
        "/c/d/e".to_string(),
        "/c/f".to_string(),
    ];
    println!("{:?}", Solution::remove_subfolders(a))
}
