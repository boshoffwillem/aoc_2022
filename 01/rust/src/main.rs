use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).unwrap();
    let elves = parse_data(contents.as_str());
    println!("Elves: {}", elves.len());
    println!("Highest calories: {}", get_top_three(elves));
}

fn parse_data(data: &str) -> Vec<Elf> {
    let mut elf_splits = data.split("\n\n");
    let mut temp = elf_splits.next();
    let mut elves: Vec<Elf> = Vec::new();

    loop {
        match temp {
            Some(x) => {
                if !x.is_empty() {
                    let mut elf = Elf::new();
                    elf.parse_calories(x);
                    elves.push(elf);
                }
            },
            None => break
        }
        temp = elf_splits.next();
    }

    elves
}

fn get_top_three(mut elves: Vec<Elf>) -> i32 {
    elves.sort_by(|a, b| a.calories.partial_cmp(&b.calories).unwrap());
    elves.reverse();
    let mut sum = 0;
    let mut i: u32 = 0;

    for elf in elves {
        if i > 2 {
            break;
        }

        sum += elf.calories;
        println!("sum: {}", elf.calories);
        i += 1;
    }

    sum
}

fn get_highest_calories(elves: Vec<Elf>) -> i32 {
    let mut highest: i32 = 0;

    for elf in elves {
        if elf.calories > highest {
            highest = elf.calories
        }
    }

    highest
}

struct Elf {
    calories: i32
}

impl Elf {
    fn new() -> Elf {
        Elf { calories: 0 }
    }

    fn parse_calories(&mut self, calories: &str) {
        let mut split = calories.split('\n');
        let mut temp = split.next();

        loop {
            match temp {
                Some(x) => {
                    let y = x.parse::<i32>();
                    match y {
                        Ok(z) => {
                            self.calories += z;
                            temp = split.next()
                        },
                        Err(_) => break
                    }
                },
                None => break
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Elf, parse_data, get_highest_calories, get_top_three};

    #[test]
    fn should_correctly_calculate_one_elfs_calories() {
        let data = "1000\n2000\n3000";
        let expected = 6000;
        let mut elf = Elf::new();
        elf.parse_calories(data);
        assert_eq!(elf.calories, expected);
    }

    #[test]
    fn should_find_all_the_elves() {
        let data = "1000\n\n2000\n3000";
        let expected_number_of_elves = 2;
        let elves = parse_data(data).len();
        assert_eq!(expected_number_of_elves, elves)
    }

    #[test]
    fn should_calculate_calories_for_multiple_elves() {
        let data = "1000\n\n2000\n3000";
        let elf_1_calories = 1000;
        let elf_2_calories = 5000;
        let elves = parse_data(data);
        assert_eq!(elf_1_calories, elves[0].calories);
        assert_eq!(elf_2_calories, elves[1].calories);
    }

    #[test]
    fn should_calculate_highest_calories() {
        let data = "1000\n\n2000\n3000\n\n7000";
        let expected = 7000;
        let elves = parse_data(data);
        let actual = get_highest_calories(elves);
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_calculate_top_three() {
        let data = "1000\n\n2000\n3000\n\n7000\n\n4000";
        let expected = 16000;
        let elves = parse_data(data);
        let actual = get_top_three(elves);
        assert_eq!(expected, actual);
    }
}
