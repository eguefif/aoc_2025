use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(&input);
    let res = solve1(&input);
    println!("part1: {res}");
    let res = solve2(&input);
    println!("part2: {res}");
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    let mut vec = vec![];

    for line in input.lines(){
        let mut splits = line.split(",");
        let n1 = splits.next().unwrap().parse::<usize>().unwrap();
        let n2 = splits.next().unwrap().parse::<usize>().unwrap();
        vec.push((n1, n2));
    }

    vec
}

fn solve1(input: &Vec<(usize, usize)>) -> usize {
    let mut area = 0;
    for p1 in input.iter() {
        for p2 in input[0..input.len()].iter() {
            if p1 == p2 {
                continue;
            }
            let temp_area = calculate_area(p1, p2);
            if temp_area > area {
                area = temp_area
            }
        }
    }
    area
}

fn calculate_area((x1, y1) : &(usize, usize), (x2, y2) : &(usize, usize)) -> usize {
    let width;
    let height;
    if x1 > x2 {
        width = x1 - x2;
    } else {
        width = x2 - x1;
    }
    if y1 > y2 {
        height = y1 - y2;
    } else {
        height = y2 - y1;
    }
    (width + 1) * (height +1)
}


fn solve2(input: &Vec<(usize, usize)>) -> usize {
    let mut area = 0;
    for p1 in input.iter() {
        for p2 in input[0..input.len()].iter() {
            if p1 == p2 || !valid_rectangle(p1, p2, input){
                continue;
            }
            let temp_area = calculate_area(p1, p2);
            if temp_area > area {
                println!("{:?} {:?}",  p1, p2);
                area = temp_area
            }
        }
    }
    area
}

fn valid_rectangle((x1, y1) : &(usize, usize), (x2, y2): &(usize, usize), input: &Vec<(usize, usize)>) -> bool {
    let p3 = (*x1, *y2);
    let p4 = (*x2, *y1);
    let mut check1 = false;
    let mut check2 = false;

    for (r1, _, r2) in input.iter().tuple_windows() {
        if is_in_rectangle(p3, *r1, *r2) {
            check1 = true;
        }
        if is_in_rectangle(p4, *r1, *r2) {
            check2 = true;
        }
        if check1 == true && check2 == true {
            return true;
        }
    }
    false
}

fn is_in_rectangle((x, y): (usize, usize), (x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> bool {
    let (x1, x2) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
    let (y1, y2) = if y1 > y2 { (y2, y1) } else { (y1, y2) };

    x >= x1 && x <= x2 && y >= y1 && y <= y2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = std::fs::read_to_string("./src/day9/test.txt").unwrap();
        let input = parse(&input);
        let res = solve1(&input);
        assert_eq!(50, res);
    }

    #[test]
    fn test_2() {
        let input = std::fs::read_to_string("./src/day9/test.txt").unwrap();
        let input = parse(&input);
        let res = solve2(&input);
        assert_eq!(24, res);
    }

    #[test]
    fn test_calculate_area() {
        let p1: (usize, usize) = (3, 4);
        let p2: (usize, usize) = (5, 8);
        let res = calculate_area(&p1, &p2);
        assert_eq!(res, 3 * 5);
    }

    #[test]
    fn test_valid_rectangle_false() {
        let input = std::fs::read_to_string("./src/day9/test.txt").unwrap();
        let input = parse(&input);
        let p1: (usize, usize) = (2, 3);
        let p2: (usize, usize) = (10, 1);
        let res = valid_rectangle(&p1, &p2, &input);
        println!("res invalid: {}", res);
        assert!(!res)
    }

    #[test]
    fn test_valid_rectangle_true() {
        let input = std::fs::read_to_string("./src/day9/test.txt").unwrap();
        let input = parse(&input);
        let p1: (usize, usize) = (9, 5);
        let p2: (usize, usize) = (2, 3);
        let res = valid_rectangle(&p1, &p2, &input);
        assert!(res)
    }

    #[test]
    fn test_is_in_rectangle() {
        let res = is_in_rectangle((3, 4), (1, 1), (5, 5));
        assert!(res);
    }

    #[test]
    fn test_is_in_rectangle2() {
        let res = is_in_rectangle((1, 4), (1, 1), (5, 5));
        assert!(res);
    }

    #[test]
    fn test_is_in_rectangle3() {
        let res = is_in_rectangle((0, 4), (1, 1), (5, 5));
        assert!(!res);
    }
}
