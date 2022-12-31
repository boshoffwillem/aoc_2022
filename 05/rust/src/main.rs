use std::env;
use std::fs;

#[derive(Debug)]
struct Move {
    count: u32,
    from: u32,
    to: u32,
}

#[derive(Debug)]
struct Stack {
    id: usize,
    elements: Vec<String>
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let data = fs::read_to_string(path).unwrap();
    let (mut stacks, moves) = parse_data(&data);

    for m in moves {
        println!("move: {:?}", m);
        let from = m.from as usize;
        let to = m.to as usize;
        let mut crates: Vec<String> = Vec::new();

        for i in 0..m.count {
            let element = stacks[from - 1].elements.remove(0);
            crates.push(element);
        }
        crates.reverse();
        for c in crates {
            stacks[to - 1].elements.insert(0, c);
        }
    }

    for stack in &stacks {
        print!("{}", stack.elements[0].replace('[', "").replace(']', ""));
    }
}

fn parse_data(data: &str) -> (Vec<Stack>, Vec<Move>) {
    let lines = data.split("\n").collect::<Vec<&str>>();
    let line_length = lines[0].len();
    let number_of_columns = line_length / 4;
    let mut stacks: Vec<Stack> = Vec::new();
    let mut moves: Vec<Move> = Vec::new();

    for i in 0..number_of_columns {
        let vec: Vec<&str> = Vec::new();
        stacks.push(Stack { id: i + 1, elements: Vec::new() });
    }

    for line in lines {
        if line.starts_with(" 1") || line.trim().is_empty() {
            continue;
        }
        else if line.starts_with("move") {
            let splits = line.split(' ').collect::<Vec<&str>>();
            let m = Move {
                from: splits[3].trim().parse::<u32>().unwrap(),
                to: splits[5].trim().parse::<u32>().unwrap(),
                count: splits[1].trim().parse::<u32>().unwrap() };
            moves.push(m);
        }
        else {
            let crates = line.as_bytes().chunks(4);
            let mut counter: u32 = 0;
            for c in crates {
                let crate_string = std::str::from_utf8(c).unwrap();
                let cleaned = crate_string.trim().to_string();

                if !cleaned.is_empty() {
                    stacks[counter as usize].elements.push(cleaned);
                }
                counter += 1;
            }
        }
    }

    (stacks, moves)
}
