fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let parsed = parse(input);
    println!("Res p1: {}", solve_p1(parsed.clone()));
    println!("Res p2: {}", solve_p2(parsed));
}

#[derive(Debug, Clone)]
enum Direction {
    Left(i32),
    Right(i32)
}

fn parse(input: String) -> Vec<Direction> {
    let mut directions: Vec<Direction> = vec![];
    for entry in input.lines(){
        let (direction, number) = entry.split_at(1);
        let n = number.parse::<i32>().expect("Error while getting n: not a number");
        if direction == "R" {
            directions.push(Direction::Right(n));
        } else {
            directions.push(Direction::Left(n));
        }
    }
    directions
}

fn solve_p1(directions: Vec<Direction>) -> i32 {
    let mut result = 0;
    let mut code = 50;
    for direction in directions {
        match direction {
            Direction::Left(v) => code = wrapping_sub(code, v),
            Direction::Right(v) => code = wrapping_sum(code, v),
        }
        if code == 0 {
            result += 1;
        }
    }
    result
}

fn wrapping_sum(a: i32, b: i32) -> i32 {
    (a + b) % 100
}

fn wrapping_sub(a :i32, b: i32) -> i32 {
    let mut res = a - b;
    while res < 0 {
        res += 100;
    }
    res
}

fn solve_p2(directions: Vec<Direction>) -> u32 {
    let mut result = 0;
    let mut code = 50;
    for direction in directions {
        let zero: u32;
        match direction {
            Direction::Left(v) => (code, zero) = wrapping_sub_zero(code, v),
            Direction::Right(v) => (code, zero) = wrapping_sum_zero(code, v),
        }
        result += zero;
    }
    result
}

fn wrapping_sum_zero(a: i32, b: i32) -> (i32, u32) {
    let flag = if a == 100 { 1 } else { 0 };
    let mut res = a + b;
    let mut zero = 0;
    while res > 99 {
        zero += 1;
        res -= 100;
    }
    (res, zero - flag)

}

fn wrapping_sub_zero(a :i32, b: i32) -> (i32, u32) {
    let flag = if a == 0 { 1 } else { 0 };
    let mut res = a - b;
    let mut zero = 0;
    while res <= 0 {
        zero += 1;
        res += 100;
    }
    (res, zero - flag)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1(){
        let input = std::fs::read_to_string("./src/day1/test.txt").unwrap();
        let directions = parse(input);
        let res = solve_p1(directions);

        assert_eq!(res, 3);
    }

    #[test]
    fn test_2(){
        let input = std::fs::read_to_string("./src/day1/test.txt").unwrap();
        let directions = parse(input);
        let res = solve_p2(directions);

        //assert_eq!(true, false);
        assert_eq!(res, 6);
    }
}
