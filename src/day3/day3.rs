fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let parsed = parse(input);
    let result = solve1(parsed);
    println!("part1: {}", result);
}

fn parse(input: String) -> Vec<Vec<u32>> {
    let mut retval: Vec<Vec<u32>> = vec![];
    for line in input.lines() {
        let mut battery: Vec<u32> = vec![];
        for a in line.chars() {
            let num: u32 = a.to_digit(10).unwrap();
            battery.push(num);
        }
        retval.push(battery);
    }
    retval
}

fn solve1(batteries: Vec<Vec<u32>>) -> u32 {
    let mut voltages: Vec<u32> = vec![];

    for battery in batteries {
        let voltage = get_voltage(battery);
        voltages.push(voltage);
    }

    voltages.into_iter().sum()
}

fn get_voltage(battery: Vec<u32>) -> u32 {
    let mut dozen = 0;
    let mut dozen_position = 0;
    let mut units = 0;
    for (i, value) in battery[0..battery.len()-1].iter().enumerate() {
        if *value > dozen {
            dozen = *value;
            dozen_position = i;
        }
    }
    for value in battery[dozen_position+1..battery.len()].iter() {
        if *value > units {
            units = *value;
        }
    }

    dozen * 10 + units
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = std::fs::read_to_string("./src/day3/test.txt").unwrap();
        let parsed = parse(input);
        let result = solve1(parsed);
        assert_eq!(357, result);
    }

    #[test]
    fn test_p1_battery() {
        let input = vec![3, 4, 5, 1, 8, 2, 5];
        let res = get_voltage(input);
        println!("res: {}", res);
        assert_eq!(85, res);
    }

    #[test]
    fn test_p1_battery_before() {
        let input = vec![8, 4, 5, 1, 2, 2, 9];
        let res = get_voltage(input);
        println!("res: {}", res);
        assert_eq!(89, res);
    }
}
