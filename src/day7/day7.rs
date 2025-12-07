use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut parsed = parse(&input);
    let res = solve1(&mut parsed, width, height);
    println!("Part1: {res}");
    // 70 is the x of the beam entry point
    let res = solve2(parsed, height, 70);
    println!("Part2: {res}");
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

fn solve1(input: &mut HashMap<(usize, usize), Value>, width: usize, height: usize) -> usize {
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

// Here is an example with the following representation
// This solution uses a hash to represent at a line how many path passes
// thourgh a given x position. We iterate thourgh all the splitter's line to
// accumulate per x position how many path. At the end, we add all the values
// we gather.
//    Rep               Hash
// ....S....     
// ....|....     {(4, 1)}
// ....^....     we split in two, there is paths through x=3, x=5. No path thourgh x=4.
// ...|.|...     {(3, 1), (4, 0), (5, 1)}
// ...^.|...     We split the beam at x=3, it goes in x=2, x=4. The beam x=5 keep going. 
// ..|.||...     {(2, 1), (4, 1), (5, 1)} 
// ..^.^|...     We split at x=2. It goes to x=1, and x=3. We split at x=4 => x=3 x=5
// .|.|.|...     {(1, 1), (2, 0), (3, 2), (4, 0), (5, 2)} 
// On the last line x=3 has 2 paths crossing coming from the two current line spliters.
// The same for x=5, one from the current splitter and one from the first splitter

fn solve2(
    input: HashMap<(usize, usize), Value>,
    height: usize,
    start: usize
) -> usize{
    // We can in memory how many times a position is used by a path in a hash.
    // The key represents the x lines.
    // The value represents the number of times thix positin is used by a path.
    let mut paths: HashMap<usize, usize> = HashMap::new();

    // At the beginning, we only have one path at position start which is the S.
    paths.insert(start, 1);

    // We iterate every 2 lines to only take a decision when there is a splitter
    for y in (1..height).step_by(2) {
        let items = paths
            .iter()
            .map(|(key, value)| (*key, *value))
            .collect::<Vec<(usize, usize)>>();
        // We iterate through all the key/value pairs
        for (x, paths_number) in items {
            // We check check if the cell under is a splitter
            if let Some(value) = input.get(&(x, y + 1)) {
                if value == &Value::Splitter {
                    // If yes, we add the value of how many path were in the current cell
                    // to the cell at x - 1 and x + 1
                    // if there was already paths passing thourgh this cell, we add the value
                    // to them
                    paths.entry(x - 1)
                        .and_modify(|value| *value += paths_number)
                        .or_insert(paths_number);

                    paths.entry(x + 1)
                        .and_modify(|value| *value += paths_number)
                        .or_insert(paths_number);
                    // The current x has no path passing through anymore
                    paths.entry(x).and_modify(|value| *value = 0);
                }
            } 
            // If not, we don't do anything, there is no new path.
        }
    }
    let mut res = 0;
    for (_key, value) in paths {
        res += value;
    }
    res
}

fn print(parsed: &HashMap<(usize, usize), Value>, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            if let Some(value) = parsed.get(&(x, y)) {
                if value == &Value::Beam {
                    print!("|");
                } else {
                    print!("^");
                }
            } else {
                    print!(".");
            }
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = std::fs::read_to_string("./src/day7/test.txt").unwrap();
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();
        let mut parsed = parse(&input);
        let res = solve1(&mut parsed, width, height);
        assert_eq!(21, res);
    }

    #[test]
    fn test_2() {
        let input = std::fs::read_to_string("./src/day7/test.txt").unwrap();
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();
        let mut parsed = parse(&input);
        let res = solve2(parsed, height, 7);
        assert_eq!(40, res);
    }
}
