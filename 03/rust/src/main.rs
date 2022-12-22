use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let data = fs::read_to_string(file_path).unwrap();
    let lines = data.split('\n');
    let mut sum = 0;
    let mut group_counter = 0;
    let mut group: Vec<&str> = Vec::new();

    for line in lines {
        group_counter += 1;
        group.push(line);

        // for c in line.chars() {
        //     if charset.contains_key(&c) {
        //         let value = charset[&c] + 1;
        //         charset.insert(c, value);
        //     }
        //     else {
        //         charset.insert(c, 1);
        //     }
        // }

        if group_counter == 3 {
            let mut charset: HashMap<char, u8> = HashMap::new();
            for item in &group {
                let mut local_chars: HashMap<char, u8> = HashMap::new();
                for c in item.chars() {
                    if c.is_whitespace() {
                        continue;
                    }
                    local_chars.insert(c, 1);
                }
                
                for (key, _) in local_chars {
                    if charset.contains_key(&key) {
                        let x = charset[&key] + 1;
                        charset.insert(key, x);

                        if x >= 3 {
                            let temp = get_priority(&key);
                            sum += temp;
                        }
                    }
                    else {
                        charset.insert(key, 1);
                    }
                }
            }

            group.clear();
            group_counter = 0;
        }
    }

    println!("Sum = {}", sum);
}

fn get_priority(value: &char) -> u32 {
    match value {
        'a' => return 1,
        'b' => return 2,
        'c' => return 3,
        'd' => return 4,
        'e' => return 5,
        'f' => return 6,
        'g' => return 7,
        'h' => return 8,
        'i' => return 9,
        'j' => return 10,
        'k' => return 11,
        'l' => return 12,
        'm' => return 13,
        'n' => return 14,
        'o' => return 15,
        'p' => return 16,
        'q' => return 17,
        'r' => return 18,
        's' => return 19,
        't' => return 20,
        'u' => return 21,
        'v' => return 22,
        'w' => return 23,
        'x' => return 24,
        'y' => return 25,
        'z' => return 26,
        'A' => return 27,
        'B' => return 28,
        'C' => return 29,
        'D' => return 30,
        'E' => return 31,
        'F' => return 32,
        'G' => return 33,
        'H' => return 34,
        'I' => return 35,
        'J' => return 36,
        'K' => return 37,
        'L' => return 38,
        'M' => return 39,
        'N' => return 40,
        'O' => return 41,
        'P' => return 42,
        'Q' => return 43,
        'R' => return 44,
        'S' => return 45,
        'T' => return 46,
        'U' => return 47,
        'V' => return 48,
        'W' => return 49,
        'X' => return 50,
        'Y' => return 51,
        'Z' => return 52,
        _ => return 0,
    }
}
