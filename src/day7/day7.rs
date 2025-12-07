use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let parsed = parse(&input);
    let res = solve1(parsed.clone(), width, height);
    println!("Part1: {res}");
}

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Splitter,
    Beam
}

fn parse(input: &str) -> HashMap<(usize, usize), Value> {
    let mut hash: HashMap<(usize, usize), Value> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                hash.insert((x, y), Value::Beam);
            } else if c == '^' {
                hash.insert((x, y), Value::Splitter);
            }
        }
    }
    hash
}

fn solve1(mut input: HashMap<(usize, usize), Value>, width: usize, height: usize) -> usize {
    let mut res = 0;
    for y in 0..(height - 1) {
        for x in 0..width {
            if let Some(value) = input.get(&(x, y)) {
                if value == &Value::Beam {
                    if let Some(value) = input.get(&(x, y + 1)) {
                        if value == &Value::Splitter {
                            res += 1;
                            if x - 1 < width && input.get(&(x - 1, y + 1)) == None {
                                input.entry((x-1, y + 1)).or_insert(Value::Beam); 
                                }
                            if x + 1 < width && input.get(&(x + 1, y + 1)) == None { 
                                input.entry((x + 1, y + 1)).or_insert(Value::Beam); 
                            }
                        } 
                    } else {
                        input.insert((x, y + 1), Value::Beam);
                    }
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = std::fs::read_to_string("./src/day7/test.txt").unwrap();
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();
        let parsed = parse(&input);
        let res = solve1(parsed.clone(), width, height);
        assert_eq!(21, res);
    }
}
