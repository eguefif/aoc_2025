fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let parsed = parse(input);
    println!("Res p1: {}", solve_p1(parsed));
}

#[derive(Debug)]
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
    let mut s = String::new();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1(){
        let input = std::fs::read_to_string("./src/day1/test.txt").unwrap();
        let directions = parse(input);
        let res = solve_p1(directions);

        assert_eq!(true, false);
        assert_eq!(res, 3);
    }
}
