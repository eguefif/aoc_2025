type RangeList = Vec<(u64, u64)>;

fn main() {
    let input = std::fs::read_to_string("./src/day2/input.txt").unwrap();
    let parsed = parse(input);
    println!("problem 1: {}", solve_p1(parsed.clone()));
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


fn solve_p1(id_ranges: RangeList) -> u64 {
    let mut valid_num: Vec<u64> = vec![];
    for range in id_ranges {
        let extension = get_valid_num_for_range(range.0, range.1);
        valid_num.extend(extension.iter());
    }
    valid_num.into_iter().sum()
}

fn get_valid_num_for_range(start: u64, end: u64) -> Vec<u64> {
    let mut retval = vec![];

    for id in start..=end {
        if is_invalid_id(id) {
            retval.push(id);
        }
    }
     
    retval
}

fn is_invalid_id(id: u64) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_solve_1() {
        let input = std::fs::read_to_string("./src/day2/test.txt").unwrap();
        let parsed = parse(input);
        assert_eq!(1227775554, solve_p1(parsed));
    }

    #[test]
    fn it_should_get_size() {
        let result = get_size(2323);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_should_get_invalid_true() {
        let result = is_invalid_id(2323);
        assert_eq!(result, true);
    }

    #[test]
    fn it_should_get_invalid_false() {
        let result = is_invalid_id(2223);
        assert_eq!(result, false);
    }
}
