use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Dir {
    N, E, S, W
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Dir::N => 'N',
            Dir::E => 'E',
            Dir::S => 'S',
            Dir::W => 'W',
        })
    }
}

impl Dir {
    fn from(c: char) -> Option<Self> {
        match c {
            'N' => Some(Dir::N),
            'E' => Some(Dir::E),
            'S' => Some(Dir::S),
            'W' => Some(Dir::W),
            _ => None
        }
    }

    fn inc(&self, pos: Pos) -> Pos {
        match self {
            Dir::N => (pos.0, pos.1-1),
            Dir::E => (pos.0+1, pos.1),
            Dir::S => (pos.0, pos.1+1),
            Dir::W => (pos.0-1, pos.1),
        }
    }
}

#[derive(Debug, Clone)]
enum Step {
    Dir(Dir),
    Branches(Vec<Vec<Step>>, bool),
}

impl Step {
    fn count(&self) -> usize {
        match self {
            Step::Dir(_) => 1,
            Step::Branches(_, true) => 0,
            Step::Branches(branches, false) => {
                branches.iter().map(|steps| steps.iter().map(|s| s.count()).sum()).max().unwrap()
            },
        }
    }
}

#[derive(Debug)]
struct Path {
    steps: Vec<Step>,
    debug: bool,
}

type Pos = (i32, i32);

impl Path {
    fn new(debug: bool) -> Self {
        Path{steps: Vec::new(), debug}
    }

    fn from(input: &str, debug: bool) -> Result<Self> {
        let reader = BufReader::new(File::open(input)?);
        let mut root = Path::new(debug);

        let line = reader.lines().next().unwrap().unwrap();
        let chars = line.chars();
        let mut iter = chars.filter(|&c| c != '$' && c != '^');

        root.parse_char_iter(&mut iter);

        Ok(root)
    }

    fn max_len(&self) -> usize {
        self.steps.iter().map(|s| s.count()).sum()
    }

    fn parse_char_iter<I>(&mut self, iter: &mut I) -> char 
    where I: Iterator<Item=char> {
        while let Some(c) = iter.next() {
            if let Some(d) = Dir::from(c) {
                self.steps.push(Step::Dir(d));
            } else if c == '(' {
                let mut branches = Vec::new();
                let mut backtracks = false;


                loop {
                    let mut subpath = Path::new(self.debug);
                    let c = subpath.parse_char_iter(iter);

                    if subpath.max_len() == 0 {
                        backtracks = true;
                    } else {
                        branches.push(subpath.steps);
                    }

                    if c == ')' {
                        break;
                    }
                }

                self.steps.push(Step::Branches(branches, backtracks));
            } else {
                return c;
            }
        }

        '$'
    }

    fn distances(&self) -> HashMap<Pos, usize> {
        let mut distances = HashMap::new();

        self.calc_distances((0, 0), 0, &vec![self.steps.clone()], &mut distances);
        if self.debug {
            let keys: Vec<&Pos> = distances.keys().collect();
            let mut sorted: Vec<(Pos, usize)> = Vec::new();
            for k in keys {
                sorted.push((*k, distances[k]));
            }
            sorted.sort_by_key(|s| s.1);
            println!("{:?}", sorted);
        }

        distances
    }

    fn calc_distances(&self, original_pos: Pos, distance: usize, branches: &Vec<Vec<Step>>, distances: &mut HashMap<Pos, usize>) -> usize {
        let mut distance = distance;

        for steps in branches {
            let mut pos = original_pos;
            for s in steps {
                match s {
                    Step::Dir(d) => {
                        pos = d.inc(pos);
                        distances.insert(pos, distance + 1);
                        distance += 1;
                    },
                    Step::Branches(branches, false) => {
                        distance = self.calc_distances(pos, distance, branches, distances);
                    },
                    Step::Branches(branches, true) => {
                        let d = self.calc_distances(pos, distance, branches, distances);
                        distance += (d - distance) / 2;
                    },
                }
            }
        }

        distance
    }
}

fn main() -> Result<()> {
    assert_eq!(Path::from("test1.input", false)?.max_len(), 10);
    assert_eq!(Path::from("test2.input", false)?.max_len(), 18);
    assert_eq!(Path::from("test3.input", false)?.max_len(), 23);
    assert_eq!(Path::from("test4.input", false)?.max_len(), 31);
    assert_eq!(Path::from("test3.input", false)?.distances().iter().filter(|d| *d.1 > 10).count(), 25);
    assert_eq!(Path::from("test4.input", false)?.distances().iter().filter(|d| *d.1 > 10).count(), 38);

    println!("Farthest door: {}", Path::from("input", false)?.max_len());
    println!("# of doors: {}", Path::from("input", true)?.distances().iter().filter(|d| *d.1 > 1000).count());

    Ok(())
}
