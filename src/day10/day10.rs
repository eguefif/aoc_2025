fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = parse(input);

    let res = solve1(&mut lines);
    println!("Part1: {}", res);
}

use std::io::{self, Write};

fn pause() {
    print!("Press Enter to continue...");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed
    
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
}

#[derive(Debug)]
struct Line {
    lights_target: Vec<bool>,
    lights: Vec<bool>,
    buttons: Vec<Vec<u8>>,
}

impl Line {
    pub fn push_button(&mut self, button_index: usize, times: u8) {
        if button_index >= self.buttons.len() {
            panic!("Button index too high");
        }
        let button = &self.buttons[button_index];
        for idx in button {
            let idx = *idx as usize;
            for _ in 0..times {
                self.lights[idx] = !self.lights[idx];
            }
        }
    }

    pub fn check(&self) -> bool {
        for (i, status) in self.lights_target.iter().enumerate() {
            if self.lights[i] != *status {
                return false
            }
        }

        true
    }
    
    pub fn reset(&mut self) {
        for light in self.lights.iter_mut() {
            *light = false;
        }
    }

    pub fn solve(&mut self) -> usize {
        let mut res = std::usize::MAX;
        if self.check() == true {
            return 0;
        }
        for n_push in 1..10 {
            let combinator = Combinator::new(n_push, self.buttons.len());
            let mut n = 0;
            for combination in combinator {
                println!("{:?}", combination);
                for (i, times) in combination.iter().enumerate() {
                    self.push_button(i, *times);
                    n += *times as usize;
                }
                if self.check() == true && res > n {
                    res = n

                }
                self.reset();
                break;
            }
        }

        res
    }
}

struct Combinator {
    n_push: u8,
    size: usize,
    current_pushes: Vec<u8>
}

impl Combinator {
    pub fn new(n_push: u8, size: usize) -> Self {
        let mut current_pushes: Vec<u8> = vec![];
        for _ in 0..size {
            current_pushes.push(0);
        }
        Self { n_push, size, current_pushes }
    }
}

impl Iterator for Combinator {
    type Item = Vec<u8>;

    // TODO: This function is the key. It should be able to return all combo possible
    fn next(&mut self) -> Option<Self::Item> {
        for i in 0..self.current_pushes.len() {
            self.current_pushes[i] += 1;
            if self.current_pushes[i] < self.n_push {
                return Some(self.current_pushes.clone());
            }
            self.current_pushes[i] = 0;
        }
        None
    }
}

fn parse(input: String) -> Vec<Line> {
    let mut retval: Vec<Line> = vec![];

    for line in input.lines() {
        let (lights_str, buttons_str) = line.split_once(' ').unwrap();

        let mut lights_target: Vec<bool> = vec![];
        for c in lights_str.chars() {
            if c == '.' {
                lights_target.push(false);
            } else if c == '#' {
                lights_target.push(true);
            } else if c ==']' {
                break;
            }
        }
        let mut buttons: Vec<Vec<u8>> = vec![];
        for button in buttons_str.split(' ') {
            let mut chars = button.chars();
            // We don't want to parse joltage
            if let Some(c) = chars.next () {
                if c == '{' {
                    break;
                }
            }
            let mut button: Vec<u8> = vec![];
            chars.for_each(|v| {
                if v != ')' && v != ',' {
                    button.push(v.to_digit(10).unwrap() as u8);
                }
            });
            buttons.push(button);
        }
        let mut lights: Vec<bool> = vec![];
        for _ in lights_target.iter() {
            lights.push(false);
        }
        let line = Line {lights_target, lights, buttons};
        retval.push(line);
    }

    retval
}


fn solve1(lines: &mut Vec<Line>) -> usize {
    let mut res = 0;
    for line in lines.iter_mut() {
        res += line.solve();
    }
    res
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_test_1() {
        let input = std::fs::read_to_string("./src/day10/test.txt").unwrap();
        let mut lines = parse(input);

        println!("{:?}", lines);
        let res = solve1(&mut lines);
        assert_eq!(7, res);
    }

    #[test]
    fn it_push_buttons() {
        let mut line = Line { lights_target: vec![true, false], lights: vec![false, false], buttons:
            vec![vec![0], vec![0, 1]]};

        line.push_button(0, 1);
        line.push_button(1, 1);
        println!("{:?}", line);
        assert_eq!(line.lights[0], false);
        assert_eq!(line.lights[1], true);
    }

    #[test]
    fn it_checks() {
        let mut line = Line { lights_target: vec![true, false], lights: vec![false, false], buttons:
            vec![vec![0], vec![0, 1]]};

        line.push_button(0, 3);
        line.push_button(1, 1);
        println!("{:?}", line);
        assert_eq!(line.check(), false);
    }

    #[test]
    fn it_checks_true() {
        let mut line = Line { lights_target: vec![true, false], lights: vec![false, false], buttons:
            vec![vec![0], vec![0, 1]]};

        line.push_button(0, 1);
        println!("{:?}", line);
        assert_eq!(line.check(), true);
    }

    #[test]
    fn it_reset() {
        let mut line = Line { lights_target: vec![true, false], lights: vec![true, false], buttons:
            vec![vec![0], vec![0, 1]]};

        line.reset();
        println!("{:?}", line);
        assert_eq!(line.lights[0], false);
        assert_eq!(line.lights[1], false);
    }
}

