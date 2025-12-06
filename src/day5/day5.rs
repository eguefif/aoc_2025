use std::cmp::Ordering;

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let parsed = parse(input);
    let res = solve1(parsed.clone());
    println!("res1: {}", res);
    let res = solve2(parsed.0);
    println!("res2: {}", res);
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

// We sort all the ranges. The goal will be to start with the range that
// has the lowest starting number.
// Then we iterate through the sorted input and try to find the biggest range we can
// build with all the overlapping range. Anytime we find an overlapping range, we update
// the end range. If the range does not overlap, it means that we won't find a bigger
// range with the current starting point. We update our result and start a new cycle.

fn solve2(mut input: Vec<(u64, u64)>) -> u64 {
    input.sort_by(|value1, value2| {
        if value1.0 < value2.0 {
            return Ordering::Less;
        } else if value1.0 == value2.0 {
            return Ordering::Equal;
        } else {
            return Ordering::Greater;
        }
    });
    
    let mut result = 0;
    let mut iter = input.iter().peekable();
    'outer: loop {
        if let Some(lower_range) = iter.next() {
            // Initial start-end range
            let start = lower_range.0;
            let mut end = lower_range.1;
            loop {
                if let Some(peek) = iter.peek() {
                    // Does the next peek overlap with the current start-end range
                    if peek.0 <= end {
                        if peek.1 > end {
                            //if the overlapping range has a higher end range, we extend
                            //our range
                            end = peek.1;
                        }
                        iter.next();
                    } else {
                        //If no overlap we update our result with the current biggest
                        //range
                        result += end - start + 1;
                        break;
                    }
                } else {
                    // we pop the last range, let's update our result
                    // with the current biggest range.
                    result += end - start + 1;
                    break 'outer;
                }
            }
        } else {
            break;
        }
    }

    result
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_day5_1() {
//        let input = std::fs::read_to_string("./src/day5/test.txt").expect("Failed to read input file");
//        let parsed = parse(input);
//        let res = solve1(parsed);
//        assert_eq!(res, 3);
//    }
//
//    #[test]
//    fn test_day5_2() {
//        let input = std::fs::read_to_string("./src/day5/test.txt").expect("Failed to read input file");
//        let parsed = parse(input);
//        let res = solve2(parsed.0);
//        assert_eq!(res, 14);
//    }
//
//    #[test]
//    fn test_day5_2_input() {
//        let input = std::fs::read_to_string("./src/day5/input.txt").expect("Failed to read input file");
//        let parsed = parse(input);
//        let res = solve2(parsed.0);
//        assert_eq!(res, 344771884978261);
//    }
//}
//
