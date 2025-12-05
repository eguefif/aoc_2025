fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let parsed = parse(input);
    let res = solve1(parsed);
    println!("res1: {}", res);
}

type Data = (Vec<(u64, u64)>, Vec<u64>);

fn parse(input: String) -> Data {
    let mut splits = input.split("\n\n");
    let ranges = splits.next().unwrap();
    let ids = splits.next().unwrap();
    let mut ret_ranges: Vec<(u64, u64)> = vec![];
    for range in ranges.lines() {
        let mut range_parts = range.split("-");
        let start = range_parts.next().unwrap().trim().parse::<u64>().unwrap();
        let end = range_parts.next().unwrap().trim().parse::<u64>().unwrap();
        ret_ranges.push((start, end));
    }
    let ret_ids = ids.lines().map(|value| value.trim().parse::<u64>().unwrap()).collect();

    (ret_ranges, ret_ids)
}

fn solve1(input: Data) -> u64 {
    input.1.iter().fold(0, |acc, id| {
        if input.0.iter().any(|(start, end)| id >= start && id <= end) {
            return acc + 1;
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5_1() {
        let input = std::fs::read_to_string("./src/day5/test.txt").expect("Failed to read input file");
        let parsed = parse(input);
        let res = solve1(parsed);
        assert_eq!(res, 3);
    }
}
