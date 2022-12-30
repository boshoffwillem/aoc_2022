use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).unwrap();
    let pairs = parse_data(&contents);

    let count = pairs.iter()
        .fold(0, |acc, &pair| {
            let mut temp = 0;
            if is_contained_pair(pair) {
                temp = 1;
            }
            acc + temp
        });
    println!("Contained: {}", count);

    let count = pairs.iter()
        .fold(0, |acc, &pair| {
            let mut temp = 0;
            if is_overlapping_range(pair) {
                temp = 1;
            }
            acc + temp
        });
    println!("Overlapping: {}", count);
}

fn parse_data(data: &str) -> Vec<&str> {
    let splits = data.split('\n');
    splits
        .filter(|split| !split.trim().is_empty())
        .collect::<Vec<&str>>()
}

fn is_overlapping_range(pair: &str) -> bool {
    let splits = pair.split(',');
    // [0] left range start
    // [1] left range end
    // [2] right range start
    // [3] right range end
    let range_bounds = splits.flat_map(|split| split.split('-')
               .map(|bound| {
                   let boundary = bound.trim().parse::<usize>();
                   match boundary {
                       Ok(value) => return value,
                       Err(err) => {
                           println!("err: {:?}", err);
                           return 0;
                       }
                   }
               }))
    .collect::<Vec<usize>>();

    let left_start = range_bounds[0];
    let left_end = range_bounds[1];
    let right_start = range_bounds[2];
    let right_end = range_bounds[3];

    left_start <= right_end && left_start >= right_start ||
        right_start >= left_start && right_start <= left_end
}

fn is_contained_pair(pair: &str) -> bool {
    let splits = pair.split(',');
    // [0] left range start
    // [1] left range end
    // [2] right range start
    // [3] right range end
    let range_bounds = splits.flat_map(|split| split.split('-')
               .map(|bound| {
                   let boundary = bound.trim().parse::<usize>();
                   match boundary {
                       Ok(value) => return value,
                       Err(err) => {
                           println!("err: {:?}", err);
                           return 0;
                       }
                   }
               }))
    .collect::<Vec<usize>>();

    let left_start = range_bounds[0];
    let left_end = range_bounds[1];
    let right_start = range_bounds[2];
    let right_end = range_bounds[3];

    if right_start >= left_start && right_end <= left_end {
        return true;
    }
    else if left_start >= right_start && left_end <= right_end {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::{parse_data, is_contained_pair, is_overlapping_range};

    #[test]
    fn should_parse_data() {
        let data = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
        "#;
        let pairs = parse_data(data);
        assert_eq!(pairs.len(), 6);
    }

    #[test]
    fn should_detect_any_overlapping_range() {
        let data = "2-4,6-8";
        let expected = false;
        let actual = is_overlapping_range(data);
        assert_eq!(actual, expected);

        let data = "2-3,4-5";
        let expected = false;
        let actual = is_overlapping_range(data);
        assert_eq!(actual, expected);

        let data = "5-7,7-9";
        let expected = true;
        let actual = is_overlapping_range(data);
        assert_eq!(actual, expected);

        let data = "2-8,3-7";
        let expected = true;
        let actual = is_overlapping_range(data);
        assert_eq!(actual, expected);

        let data = "6-6,4-6";
        let expected = true;
        let actual = is_overlapping_range(data);
        assert_eq!(actual, expected);

        let data = "2-6,4-8";
        let expected = true;
        let actual = is_overlapping_range(data);
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_detect_non_contained_right_pair() {
        let expected = false;
        let data = "2-4,6-8";
        let actual = is_contained_pair(data);
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_detect_non_contained_left_pair() {
        let expected = false;
        let data = "6-8,2-4";
        let actual = is_contained_pair(data);
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_detect_right_contained_pair() {
        let expected = true;
        let data = "2-8,3-7";
        let actual = is_contained_pair(data);
        assert_eq!(actual, expected);

        let data = "6-6,4-6";
        let actual = is_contained_pair(data);
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_detect_left_contained_pair() {
        let expected = true;
        let data = "3-7,2-8";
        let actual = is_contained_pair(data);
        assert_eq!(actual, expected);

        let data = "4-6,6-6";
        let actual = is_contained_pair(data);
        assert_eq!(actual, expected);
    }
}
