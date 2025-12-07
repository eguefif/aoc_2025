use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let parsed = parse(input.clone());
    let res = solve1(parsed.clone());
    println!("Part1: {}", res);
    let operands = parse2(parsed.0, input);
    let res = solve2((operands, parsed.1));
    println!("Part2: {}", res);
}

fn parse(input: String) -> (HashMap<usize, Vec<u64>>, Vec<char>) {
    let mut hash: HashMap<usize, Vec<u64>> = HashMap::new();
    let mut operators: Vec<char> = vec![];

    let iter = input.lines();
    for line in iter {
        let mut counter = 0;
        for value in line.split(" ") {
            if value.len() == 0 {
                continue;
            }
            if let Ok(number) = value.parse::<u64>() {
                hash.entry(counter).and_modify(|numbers| numbers.push(number)).or_insert(vec![number]);
                counter += 1;
            } else {
                operators.push(value.chars().nth(0).unwrap());
            }
        }
    }
    (hash, operators)
}

fn solve1((operands, operators) : (HashMap<usize, Vec<u64>>, Vec<char>)) -> u64 {
    let mut result = 0;
    for (key, operands) in operands {
        match operators[key] {
            '+' => result += operands.iter().sum::<u64>(),
            '*' => result += operands.iter().product::<u64>(),
            _ => panic!(),
        }
    }
    result
}

fn parse2(original_hash: HashMap<usize, Vec<u64>>, input: String) -> HashMap<usize, Vec<u64>> {
    let mut hash: HashMap<usize, Vec<u64>> = HashMap::new();
    for line in input.lines() {
        let mut counter = 0;
        let mut remaining = line;
        loop {
            let Some(numbers) = original_hash.get(&counter) else {
                break;
            };
            // We will get number string based on the largest number in the col
            let widest_number = get_size_from_vec(numbers);
            let (value, part2) = remaining.split_at(widest_number);
            let Ok(num) = value.trim_start().replace(" ", "0").parse::<u64>() else {
                break;
            };
            hash.entry(counter).and_modify(|v| v.push(num)).or_insert(vec![num]);

            // Removes the white space if any
            if part2.len() > 0 {
                let (_, part2) = part2.split_at(1);
                remaining = part2;
            }
            counter += 1;
        }
    }
    hash
}

fn solve2((operands, operators) : (HashMap<usize, Vec<u64>>, Vec<char>)) -> u64 {
    let mut result = 0;
    for (key, operands) in operands {
        let operands = pre_process_operands(operands);
        match operators[key] {
            '+' => result += operands.iter().sum::<u64>(),
            '*' => {
                result += operands
                    .iter()
                    .filter(|v| **v != 0)
                    .product::<u64>();
            },
            _ => panic!(),
        }
    }
    result
}

fn pre_process_operands(operands: Vec<u64>) -> Vec<u64> {
    let mut retval: Vec<u64> = vec![];
    let size = operands.len();
    for i in 0..size {
        let mut new_value = 0;
        for num in operands.iter() {
            let pow = 10u64.pow(i as u32);
            let digit = num / pow % 10;
            if digit != 0 {
                new_value = new_value * 10 + digit
            }
        }
        retval.push(new_value)
    }
    retval
}

fn get_size_from_vec(operands: &Vec<u64>) -> usize {
    let mut size = 0;
    for operand in operands {
        let next_size = get_size(*operand);
        if next_size > size {
            size = next_size;
        }
    }
    size
}

fn get_size(mut number: u64) -> usize {
    let mut size = 0;
    loop {
        size += 1;
        number /= 10;
        if number == 0 {
            break;
        }
    }
    size
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_example_part1() {
//        let input = std::fs::read_to_string("./src/day6/test.txt").unwrap();
//        let parsed = parse(input);
//        let res = solve1(parsed);
//        assert_eq!(res, 4277556);
//    }
//
//    #[test]
//    fn test_example_part2() {
//        let input = std::fs::read_to_string("./src/day6/test.txt").unwrap();
//        let parsed = parse(input.clone());
//        let operands = parse2(parsed.0, input);
//        let res = solve2((operands, parsed.1));
//        assert_eq!(res, 3263827);
//    }
//}
