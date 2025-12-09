fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(&input);
    let res = solve1(&input);
    println!("part1: {res}");
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
    fn test_calculate_area() {
        let p1: (usize, usize) = (3, 4);
        let p2: (usize, usize) = (5, 8);
        let res = calculate_area(&p1, &p2);
        assert_eq!(res, 3 * 5);
    }
}
