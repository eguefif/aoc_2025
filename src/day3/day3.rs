fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let parsed = parse(input);
    let result = solve1(parsed.clone());
    println!("part1: {}", result);

    let result = solve2(parsed);
    println!("part1: {}", result);
}

fn parse(input: String) -> Vec<Vec<u64>> {
    let mut retval: Vec<Vec<u64>> = vec![];
    for line in input.lines() {
        let mut battery: Vec<u64> = vec![];
        for a in line.chars() {
            let num: u32 = a.to_digit(10).unwrap();
            battery.push(num as u64);
        }
        retval.push(battery);
    }
    retval
}

fn solve1(batteries: Vec<Vec<u64>>) -> u64 {
    let mut voltages: Vec<u64> = vec![];

    for battery in batteries {
        let voltage = get_voltage(battery, 2);
        voltages.push(voltage);
    }

    voltages.into_iter().sum()
}

fn solve2(batteries: Vec<Vec<u64>>) -> u64 {
    let mut voltages: Vec<u64> = vec![];

    for battery in batteries {
        let voltage = get_voltage(battery, 12);
        voltages.push(voltage);
    }

    voltages.into_iter().sum()
}

fn get_voltage(battery: Vec<u64>, size: usize) -> u64 {
    let mut voltage: u64 = 0;
    let mut position = 0;
    let mut start_range = 0;

    for i in 0..size {
        let mut current_value = 0;
        let end_range = battery.len() - (size - i);
        start_range = position + start_range;
        if start_range >= battery.len() {
            break
        }
        for (j, value) in battery[start_range..=end_range].iter().enumerate() {
            if *value > current_value {
                current_value = *value;
                position = j + 1;
            }
        }
        voltage += 10u64.pow((size - i - 1) as u32) * current_value;
    }

    voltage
}

//#[cfg(test)]
//
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_p1() {
//        let input = std::fs::read_to_string("./src/day3/test.txt").unwrap();
//        let parsed = parse(input);
//        let result = solve1(parsed);
//        assert_eq!(357, result);
//    }
//
//    #[test]
//    fn test_p2() {
//        let input = std::fs::read_to_string("./src/day3/test.txt").unwrap();
//        let parsed = parse(input);
//        let result = solve2(parsed);
//        assert_eq!(3121910778619, result);
//    }
//
//    #[test]
//    fn test_p1_with_input() {
//        let input = std::fs::read_to_string("./src/day3/input.txt").unwrap();
//        let parsed = parse(input);
//        let result = solve1(parsed);
//        assert_eq!(17412, result);
//    }
//
//    #[test]
//    fn test_p2_with_input() {
//        let input = std::fs::read_to_string("./src/day3/input.txt").unwrap();
//        let parsed = parse(input);
//        let result = solve2(parsed);
//        assert_eq!(172681562473501, result);
//    }
//
//    #[test]
//    fn test_p1_battery() {
//        let input = vec![3, 4, 5, 1, 8, 2, 5];
//        let res = get_voltage(input, 2);
//        println!("res: {}", res);
//        assert_eq!(85, res);
//    }
//
//    #[test]
//    fn test_p1_battery_before() {
//        let input = vec![8, 4, 5, 1, 2, 2, 9];
//        let res = get_voltage(input, 2);
//        println!("res: {}", res);
//        assert_eq!(89, res);
//    }
//}
