use std::{env::args, fs, collections::HashSet};

fn main() {
    let args: Vec<String> = args().collect();
    let path = &args[1];
    let data = fs::read_to_string(path).expect("File not found");
    let mut last_four: HashSet<char> = HashSet::new();
    last_four.reserve(4);
    let chars: Vec<char> = data.chars().collect();

    // hashset
    for i in 0..chars.len() - 14 {
        last_four.insert(chars[i]);
        for j in (i + 1)..(i + 14) {
            let result = last_four.insert(chars[j]);
            if !result {
                break;
            }
        }
        if last_four.len() == 14 {
            println!("hashset: {}", i + 14);
            break;
        }
        last_four.clear();
    }
}
