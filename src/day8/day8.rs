use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut parsed = parse(input);
    let res = solve1(&mut parsed);
    println!("Part1: {}", res);
}

#[derive(Debug, Clone)]
struct Block {
    x: i64,
    y: i64,
    z: i64,
    block1: Option<usize>,
    block2: Option<usize>
}

impl Block {
    pub fn new(input: &str) -> Self {
        let mut splits = input.split(",");
        Self {
            x: splits.next().unwrap().parse::<i64>().unwrap(),
            y: splits.next().unwrap().parse::<i64>().unwrap(),
            z: splits.next().unwrap().parse::<i64>().unwrap(),
            block1: None,
            block2: None
        }
    }

    fn set_nearests(&mut self, id1: usize, id2: usize) {
        self.block1 = Some(id1);
        self.block2 = Some(id2);
    }

    fn set_nearest(&mut self, id: usize) {
        if self.block1 == None {
            self.block1 = Some(id);
            return;
        }
        if self.block2 == None {
            self.block2 = Some(id);
            return;
        }
        //if !(Some(id) == self.block1 || Some(id) == self.block2) { panic!(); }
    }
}

fn distance(block1: &Block, block2: &Block) -> f64 {
    ((block1.x - block2.x).pow(2) as f64 + (block1.y - block2.y).pow(2) as f64 + (block1.z - block2.z).pow(2) as f64).sqrt()

}

fn parse(input: String) -> HashMap<usize, Block> {
    let mut retval: HashMap<usize, Block> = HashMap::new();
    for (id, line) in input.lines().enumerate() {
        retval.insert(id, Block::new(line));
    }
    retval
}

fn solve1(blocks: &mut HashMap<usize, Block>) -> usize {
    build_nearest(blocks);
    let graphs = build_graphs(blocks);
    let mut res = 0;
    for graph in graphs {
        println!("{:?}", graph);
        res += graph.len();
    }
    res
}

fn build_nearest(blocks: &mut HashMap<usize, Block>) {
    let size = blocks.len();
    for id in 0..(size - 2) {
        let (id1, id2) = get_nearest(id, blocks);
        blocks.entry(id).and_modify(|value| value.set_nearests(id1, id2));
        blocks.entry(id1).and_modify(|value| value.set_nearest(id));
        blocks.entry(id2).and_modify(|value| value.set_nearest(id));
    }
}

fn get_nearest(id: usize, blocks: &mut HashMap<usize, Block>) -> (usize ,usize) {
    let mut distances: Vec<(usize, f64)> = vec![];
    let block1 = blocks.get(&id).unwrap();
    for id2 in (id + 1)..blocks.len() {
        let block2 = blocks.get(&id2).unwrap();
        distances.push((id2, distance(block1, block2)));
    }
    distances.sort_by(|a, b| {
        if a.1 == b.1 { return Ordering::Equal; }
        else if a.1 < b.1 { return Ordering::Less; }
        else { return Ordering::Greater; }
    });
    (distances[0].0, distances[1].0)
}

fn build_graphs(blocks: &mut HashMap<usize, Block>) -> Vec<Vec<usize>> {
    let mut graphs: Vec<Vec<usize>> = vec![];
    let ids = blocks.keys().collect::<Vec<&usize>>();
    'outer: for id in ids {
        for graph in graphs.iter() {
            if graph.contains(id) {
                continue 'outer;
            }
        }
        let mut graph = vec![];
        build_graph(*id, blocks, &mut graph);
        graphs.push(graph);
    }

    graphs
}

fn build_graph(id: usize, blocks: &HashMap<usize, Block>, graph: &mut Vec<usize>){
    graph.push(id);
    let block = blocks.get(&id).unwrap();
    let id2 = block.block1.unwrap();
    let id3 = block.block2.unwrap();
    if !graph.contains(&id2) {
        build_graph(id2, blocks, graph);
    }
    if !graph.contains(&id3) {
        build_graph(id3, blocks, graph);
    }
}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_1() {
//        let input = std::fs::read_to_string("./src/day8/test.txt").unwrap();
//        let mut parsed = parse(input);
//        let res = solve1(&mut parsed);
//        assert_eq!(res, 40);
//    }
//
//    #[test]
//    fn test_distance() {
//        let block1 = Block::new("315,211,147");
//        let block2 = Block::new("412,120,320");
//        let dist = distance(&block1, &block2);
//        assert!(dist < 218.2178);
//        assert!(dist > 218.217);
//    }
//}
