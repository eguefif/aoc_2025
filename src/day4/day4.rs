use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let parsed = parse(input);
    let res = solve1(parsed.clone());
    println!("Res!: {}", res);
    let res = solve2(parsed);
    println!("Res!: {}", res);
}

fn parse(input: String) -> HashSet<(i32, i32)> {
    let mut retval: HashSet<(i32, i32)> = HashSet::new();
    for (y, line) in input.lines().enumerate() {
       for (x, value) in line.chars().enumerate() {
            if value == '@' {
                retval.insert((x as i32, y as i32));
            }
       }
    }
    retval
}

fn solve1(input: HashSet<(i32, i32)>) -> u32 {
    let mut res = 0;
    for roll in input.iter() {
        if check(roll, &input) {
            res += 1;
        }

    }
    res
}

fn solve2(input: HashSet<(i32, i32)>) -> u32 {
    let mut res = 0;
    let mut size = input.len();
    let mut new_set = input.clone();
    loop {
        new_set = trim_down(new_set);
        if size - new_set.len() == 0 {
            break;
        }
        res += size - new_set.len();
        size = new_set.len();
    }
    res as u32
}

fn trim_down(set: HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut new_set: HashSet<(i32, i32)> = HashSet::new();
    
    for entry in set.iter() {
        if !check(entry, &set) {
            new_set.insert(*entry);
        }
    }

    new_set
}

fn check((x, y): &(i32, i32), input: &HashSet::<(i32, i32)>) -> bool {
    let mut res = 0;
    if input.contains(&(*x, *y + 1)) { res += 1 }
    if input.contains(&(*x + 1, *y + 1)) { res += 1 }
    if input.contains(&(*x + 1, *y)) { res += 1 }
    if input.contains(&(*x + 1, *y - 1)) { res += 1 }
    if input.contains(&(*x, *y - 1)) { res += 1 }
    if input.contains(&(*x - 1, *y - 1)) { res += 1 }
    if input.contains(&(*x - 1, *y)) { res += 1 }
    if input.contains(&(*x - 1, *y + 1)) { res += 1 }

    res < 4
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_example() {
//        let input = std::fs::read_to_string("./src/day4/test.txt").unwrap();
//        let parsed = parse(input);
//        let res = solve1(parsed);
//        assert_eq!(res, 13);
//    }
//
//    #[test]
//    fn test_example2() {
//        let input = std::fs::read_to_string("./src/day4/test.txt").unwrap();
//        let parsed = parse(input);
//        let res = solve2(parsed);
//        assert_eq!(res, 43);
//    }
//}
