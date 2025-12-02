type RangeList = Vec<(u64, u64)>;

fn main() {
    let input = std::fs::read_to_string("./src/day2/input.txt").unwrap();
    let parsed = parse(input);
    println!("problem 1: {}", solve(parsed.clone(), is_invalid_id_1));
    println!("problem 2: {}", solve(parsed.clone(), is_invalid_id_2));
}

fn parse(input: String) -> RangeList {
    let mut retval: RangeList = vec![];
    for range in input.split(',') {
        let mut values = range.split('-');
        let start = values.next().unwrap().trim().parse::<u64>().unwrap();
        let end = values.next().unwrap().trim().parse::<u64>().unwrap();
        retval.push((start, end));
    }
    retval
}


fn solve<T: Fn(u64) -> bool>(id_ranges: RangeList, fct: T) -> u64 {
    let mut invalid_num: Vec<u64> = vec![];
    for range in id_ranges {
        for id in range.0..=range.1 {
            if fct(id) {
                invalid_num.push(id);
            }
        }
    }
    invalid_num.into_iter().sum()
}

fn is_invalid_id_1(id: u64) -> bool {
    let size = get_size(id);
    if size % 2 != 0 {
        return false
    }
    let first_half = id % (10u64.pow(size / 2));
    let second_half = id / (10u64.pow(size / 2));
    first_half == second_half
}

fn get_size(mut id: u64) -> u32 {
    let mut size = 1;
    loop {
        if id / 10 == 0 {
            break
        }
        id = id / 10;
        size += 1;
    }
    size
}

fn is_invalid_id_2(id: u64) -> bool {
    let id = id.to_string();
    let mut pattern_size = 1;
    'outer: loop {
        if id.len() / 2 < pattern_size {
            return false;
        }
        let mut iter = id.as_bytes().chunks(pattern_size);
        if let Some(pattern) = iter.next() {
            for chunk in iter {
                if chunk != pattern {
                    pattern_size += 1;
                    continue 'outer;
                }
            }
            return true;
        } else {
            return false;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_solve_1() {
        let input = std::fs::read_to_string("./src/day2/test.txt").unwrap();
        let parsed = parse(input);
        assert_eq!(1227775554, solve(parsed, is_invalid_id_1));
    }

    #[test]
    fn it_should_solve_1_real_input() {
        let input = std::fs::read_to_string("./src/day2/input.txt").unwrap();
        let parsed = parse(input);
        assert_eq!(29940924880, solve(parsed, is_invalid_id_1));
    }

    #[test]
    fn it_should_solve_2_real_input() {
        let input = std::fs::read_to_string("./src/day2/test.txt").unwrap();
        let parsed = parse(input);
        assert_eq!(4174379265, solve(parsed, is_invalid_id_2));
    }

    #[test]
    fn it_should_get_size() {
        let result = get_size(2323);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_should_get_invalid_true() {
        let result = is_invalid_id_1(2323);
        assert_eq!(result, true);
    }

    #[test]
    fn it_should_get_invalid_false() {
        let result = is_invalid_id_1(2223);
        assert_eq!(result, false);
    }

    #[test]
    fn it_should_get_invalid_2_true() {
        let result = is_invalid_id_2(232323);
        assert_eq!(result, true);
    }

    #[test]
    fn it_should_get_invalid_2_false() {
        let result = is_invalid_id_2(232321);
        assert_eq!(result, false);
    }

    #[test]
    fn it_should_get_invalid_2_false_1111() {
        let result = is_invalid_id_2(1111);
        assert_eq!(result, true);
    }

    #[test]
    fn it_should_get_invalid_2_false_824824824() {
        let result = is_invalid_id_2(824824824);
        assert_eq!(result, true);
    }

    #[test]
    fn it_should_get_invalid_2_false_123123123() {
        let result = is_invalid_id_2(123123123);
        assert_eq!(result, true);
    }

    #[test]
    fn it_should_get_invalid_2_false_12341234() {
        let result = is_invalid_id_2(12341234);
        assert_eq!(result, true);
    }

    #[test]
    fn it_should_get_invalid_2_false_1() {
        let result = is_invalid_id_2(1);
        assert_eq!(result, false);
    }

    #[test]
    fn it_should_get_invalid_2_true_22() {
        let result = is_invalid_id_2(22);
        assert_eq!(result, true);
    }
}
