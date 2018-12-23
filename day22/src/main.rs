use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

#[derive(Copy, Clone, Eq, PartialEq)]
enum Type {
    Rocky, Wet, Narrow
}

impl Type {
    fn risk(&self) -> usize {
        match self {
            Type::Rocky => 0,
            Type::Wet => 1,
            Type::Narrow => 2,
        }
    }
}

type Pos = (usize, usize);

trait Geology {
    fn geo_index(&self, cave: &mut Cave) -> usize;
    fn erosion_level(&self, cave: &mut Cave) -> usize;
    fn kind(&self, cave: &mut Cave) -> Type;
}

impl Geology for Pos {
    fn geo_index(&self, cave: &mut Cave) -> usize {
        if *self == (0, 0) || *self == cave.target {
            return 0
        }

        if self.1 == 0 {
            return self.0 * 16807
        }

        if self.0 == 0 {
            return self.1 * 48271
        }

        &(self.0-1, self.1).erosion_level(cave) * &(self.0, self.1-1).erosion_level(cave)
    }

    fn erosion_level(&self, cave: &mut Cave) -> usize {
        if !cave.erosion.contains_key(self) {
            let erosion = (self.geo_index(cave) + cave.depth) % 20183;
            cave.erosion.insert(*self, erosion);
        }

        cave.erosion[self]
    }

    fn kind(&self, cave: &mut Cave) -> Type {
        match self.erosion_level(cave) % 3 {
            0 => Type::Rocky,
            1 => Type::Wet,
            2 => Type::Narrow,
            _ => panic!("Impossible match")
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    pos: Pos,
    tool: Tool,
    dist: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Tool {
    Torch, Climb, Neither,
}

impl Ord for State {
	fn cmp(&self, other: &State) -> Ordering {
		other.cost.cmp(&self.cost).then_with(|| other.dist.cmp(&self.dist))
	}
}

impl PartialOrd for State {
	fn partial_cmp(&self, other: &State) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

#[derive(Debug)]
struct Cave {
    depth: usize,
    target: Pos,
    erosion: HashMap<Pos, usize>,
}

impl Cave {
    fn new(depth: usize, target: Pos) -> Self {
        Cave{depth, target, erosion: HashMap::new()}
    }

    fn from(input: &str) -> Result<Self> {
        let reader = BufReader::new(File::open(input)?);
        let mut depth = 0;
        let mut target = (0, 0);

        for line in reader.lines() {
            let line = line.unwrap();
            if line.starts_with("depth: ") {
                depth = line[7..].parse().unwrap();
            } else if line.starts_with("target: ") {
                let mut parts = line[8..].split(",");
                target = (parts.next().unwrap().parse().unwrap(), parts.next().unwrap().parse().unwrap());
            }
        }

        Ok(Self::new(depth, target))
    }

    fn risk_to_target(&mut self) -> usize {
        let mut risk = 0;

        for y in 0..=self.target.1 {
            for x in 0..=self.target.0 {
                risk += &(x, y).kind(self).risk();
            }
        }

        risk
    }

    fn shortest_path(&mut self) -> usize {
        let mut dist = HashMap::new();
        let mut heap = BinaryHeap::new();
        let start = (0, 0);

        dist.insert((start, Tool::Torch), 0);
        heap.push(State{cost: 0, dist: 0, pos: start, tool: Tool::Torch});

        while let Some(State{ cost, pos, tool, dist: _ }) = heap.pop() {
            if pos == self.target {
                return cost;
            }

            if cost > dist.get(&(pos, tool)).map(|&v| v).unwrap_or(usize::max_value()) {
                continue;
            }

            for p in self.adjacent(pos) {
                let kind = p.kind(self);
                for alt in vec![false, true] {
                    let (ptool, pcost) = cost_and_tool(tool, kind, self.target == p, alt);

                    let next = State{cost: cost + pcost, pos: p, tool: ptool, dist: dist_to(p, self.target)};

                    if next.cost < dist.get(&(p, ptool)).map(|&v| v).unwrap_or(usize::max_value()) {
                        heap.push(next);
                        dist.insert((p, ptool), next.cost);
                    }
                }
            }
        }

        unreachable!()
    }

    fn adjacent(&self, pos: Pos) -> Vec<Pos> {
        let mut adj = Vec::new();

        // Up
        if pos.1 > 0 {
            adj.push((pos.0, pos.1-1))
        }

        // Left
        if pos.0 > 0 {
            adj.push((pos.0-1, pos.1))
        }

        // Right
        adj.push((pos.0+1, pos.1));

        // Down
        adj.push((pos.0, pos.1+1));

        adj
    }
}

fn main() -> Result<()> {
    assert_eq!(Cave::from("test1.input")?.risk_to_target(), 114);

    println!("Risk to target: {}", Cave::from("input")?.risk_to_target());

    assert_eq!(Cave::from("test1.input")?.shortest_path(), 45);

    println!("Shortest path: {:?}", Cave::from("input")?.shortest_path());
    Ok(())
}

fn cost_and_tool(tool: Tool, kind: Type, target: bool, alt: bool) -> (Tool, usize) {
    let cost = match kind {
        Type::Rocky => if tool == Tool::Neither { if alt { (Tool::Torch, 8) } else { (Tool::Climb, 8) } } else { (tool, 1) },
        Type::Wet => if tool == Tool::Torch { if alt { (Tool::Climb, 8) } else { (Tool::Neither, 8) } } else { (tool, 1) },
        Type::Narrow => if tool == Tool::Climb { if alt { (Tool::Neither, 8) } else { (Tool::Torch, 8) } } else { (tool, 1) },
    };

    if target && cost.0 != Tool::Torch {
        return (Tool::Torch, 8)
    }

    cost
}

fn dist_to(p: Pos, t: Pos) -> usize {
    ((p.0 as i32 - t.0 as i32).abs() + (p.1 as i32 - t.1 as i32).abs()) as usize
}
