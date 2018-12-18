use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;
use std::iter::FromIterator;
use std::collections::{HashMap, HashSet};
use std::collections::VecDeque;
use std::ops::RangeInclusive as RangeI;

struct Cavern {
    clay: HashMap<usize, HashSet<usize>>,
    water: HashMap<Pos, WaterType>,
    debug: bool,
}

#[derive(PartialEq, Copy, Clone)]
enum WaterType {
    Flowing,
    Settled,
}

enum Flow {
    Down,
    Left,
    Right,
    Up,
}

type Pos = (usize, usize);

impl fmt::Display for Flow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Flow::Down => "down",
            Flow::Up => "up",
            Flow::Right => "right",
            Flow::Left => "left",
        };

        write!(f, "{}", str)
    }
}

impl fmt::Display for Cavern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut keys : Vec<&usize> = self.clay.keys().collect();
        keys.sort();
        let max_y = **keys.iter().max().unwrap();

        let (mut min_x, mut max_x) = (1<<32, 0);
        for y in keys {
            let mut xs: Vec<&usize> = self.clay[y].iter().collect();
            xs.sort();
            if xs[0] < &min_x {
                min_x = *xs[0];
            }

            if *xs.last().unwrap() > &max_x {
                max_x = **xs.last().unwrap();
            }
        }


        for y in 0..=max_y {
            let temp = HashSet::new();
            let xs = self.clay.get(&y).unwrap_or(&temp);
            //write!(f, "{}:\t\t", y)?;
            for x in min_x..=max_x {
                if y == 0 && x == 500 {
                    write!(f, "+")?;
                }
                if self.water.contains_key(&(x, y)) {
                    if WaterType::Flowing == self.water[&(x, y)] {
                        write!(f, "|")?;
                    } else {
                        write!(f, "~")?;
                    }
                } else if xs.contains(&x) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Cavern {
    fn new(clay: HashMap<usize, HashSet<usize>>, debug: bool) -> Cavern {
        Cavern{clay, water: HashMap::new(), debug}
    }

    fn from(input: &str, debug: bool) -> Result<Cavern> {
        let reader = BufReader::new(File::open(input)?);
        let mut data: HashMap<usize, HashSet<usize>> = HashMap::new();

        for line in reader.lines() {
            let line = line.unwrap();
            let parts : Vec<&str> = line.split(", ").flat_map(|l| l.split("=")).collect();
            if parts[0] == "x" {
                let x: usize = parts[1].parse().unwrap();
                let yrange: Vec<usize> = parts[3].split("..").map(|v| v.parse().unwrap()).collect();
                (yrange[0]..=yrange[1]).for_each(|y| {
                    if let Some(v) = data.get_mut(&y) {
                        v.insert(x);
                    } else {
                        let mut v = HashSet::new();
                        v.insert(x);
                        data.insert(y, v);
                    }
                });
            } else {
                let y: usize = parts[1].parse().unwrap();
                let xrange: Vec<usize> = parts[3].split("..").map(|v| v.parse().unwrap()).collect();
                (xrange[0]..=xrange[1]).for_each(|x| {
                    if let Some(v) = data.get_mut(&y) {
                        v.insert(x);
                    } else {
                        let mut v = HashSet::new();
                        v.insert(x);
                        data.insert(y, v);
                    }
                });
            }
        }

        Ok(Cavern::new(data, debug))
    }

    fn water(&mut self) -> HashMap<Pos, WaterType> {
        let min_y = 0;
        let max_y = self.clay.keys().max().unwrap();

        self.trace_stream((500, min_y), Flow::Down, *max_y, None);

        self.water.clone()
    }

    fn trace_stream(&mut self, pos: Pos, dir: Flow, max_y: usize, floor: Option<RangeI<usize>>) {
        let mut deque = VecDeque::new();
        deque.push_back((pos, dir, floor));

        let mut iter = -1;
        while deque.len() > 0 {
            let (pos, dir, floor) = deque.pop_front().unwrap();
            iter += 1;

            if self.debug && iter % 20 == 0 {
                println!("Current cavern: {}", self.water.len());
                println!("{}", self);
            }

            match dir {
                Flow::Left | Flow::Right => {
                    if floor.is_none() {
                        continue
                    }

                    let x = if let Flow::Left = dir { pos.0 - 1 } else { pos.0 + 1 };
                    if self.clay.get(&pos.1).map(|xs| xs.contains(&x)).unwrap_or(false) {
                        let up = (pos.0, pos.1-1);
                        let floor = self.floor(up);
                        if self.container(pos, floor.clone()).is_some() {
                            deque.push_back((up, Flow::Up, floor));
                        }
                        continue 
                    }

                    let floor = floor.unwrap();
                    let (start, end) = (*floor.start(), *floor.end());

                    if start <= pos.0 && end >= pos.0 {
                        let pos = (x, pos.1);
                        self.water.insert(pos, WaterType::Flowing);
                        if self.debug {
                            println!("Flowing {} at {:?}", dir, pos);
                        }

                        deque.push_back((pos, dir, Some(floor.clone())));

                        // Overflow to the side
                        if start > x || end < x {
                            deque.push_back((pos, Flow::Down, Some(floor)));
                        }
                    }
                },
                Flow::Down => {
                    if pos.1+1 > max_y || self.water.contains_key(&(pos.0, pos.1+1)) {
                        continue
                    }

                    if self.clay.get(&(pos.1+1)).map(|xs| xs.contains(&pos.0)).unwrap_or(false) {
                        // Clay underneath
                        deque.push_back((pos, Flow::Up, None));
                    } else {
                        let pos = (pos.0, pos.1+1);
                        self.water.insert(pos, WaterType::Flowing);
                        if self.debug {
                            println!("Flowing {} at {:?}", dir, pos);
                        }
                        deque.push_back((pos, Flow::Down, None));
                    }
                },
                Flow::Up => {
                    let floor = floor.or_else(|| self.floor(pos));
                    if let Some(range) = self.container(pos, floor.clone()) {
                        if self.debug {
                            println!("Filling container of {:?} at {:?}", range, pos);
                        }
                        range.clone().for_each(|x| {self.water.insert((x, pos.1), WaterType::Settled);});
                        let pos = (pos.0, pos.1-1);
                        let floor = RangeI::new(range.start()-1, range.end()+1);
                        deque.push_back((pos, Flow::Up, Some(floor)));
                    } else {
                        // Overflows
                        if self.debug {
                            println!("Overflowing at {:?}", pos);
                        }
                        self.water.insert(pos, WaterType::Flowing);
                        deque.push_back((pos, Flow::Left, floor.clone()));
                        deque.push_back((pos, Flow::Right, floor));
                    }
                },
            }
        }

        if self.debug {
            println!("Current cavern: {}", self.water.len());
            println!("{}", self);
        }
    }

    fn floor(&self, pos: Pos) -> Option<RangeI<usize>> {
        if let Some(xs) = self.clay.get(&(pos.1+1)) {
            if xs.contains(&(pos.0)) {
                let (mut min_x, mut max_x) = (None, None);
                let mut i = 1;
                loop {
                    if min_x.is_none() && !xs.contains(&(pos.0-i)) {
                        min_x = Some(pos.0-i+1);
                    }

                    if max_x.is_none() && !xs.contains(&(pos.0+i)) {
                        max_x = Some(pos.0+i-1);
                    }

                    if min_x.is_some() && max_x.is_some() {
                        break
                    }

                    i+=1;
                }

                return Some((min_x.unwrap())..=(max_x.unwrap()));
            } else {
                let mut y = pos.1 + 1;
                let mut row = Vec::from_iter(xs.iter().cloned());
                row.sort();
                let mut min_x = 0;
                for x in row {
                    if pos.0 > x {
                        min_x = x;
                    } else {
                        let max_x = x;
                        while self.clay.get(&(y+1)).map(|xs| xs.contains(&min_x) && xs.contains(&max_x)).unwrap_or(false) {
                            y += 1;
                        }
                        let xs = self.clay.get(&y).unwrap();
                        for x in min_x..=max_x {
                            if !xs.contains(&x) {
                                return None
                            }
                        }
                        println!("{}x{}", min_x, max_x);
                        return Some(min_x..=max_x);
                    }
                }
            }
        } 

        None
    }

    fn container(&self, pos: Pos, floor: Option<RangeI<usize>>) -> Option<RangeI<usize>> {
        if self.clay.get(&(pos.1)).is_none() {
            return None;
        }

        if floor.is_none() {
            return None;
        }

        let floor = floor.unwrap();
        let xs = self.clay.get(&(pos.1)).unwrap();

        let (start, end) = (floor.start(), floor.end());
        let (mut min_x, mut max_x) = (None, None);
        let mut i = 1;
        // Find the walls
        loop {
            if min_x.is_none() && xs.contains(&(pos.0-i)) {
                min_x = Some(pos.0-i+1);
            }

            if max_x.is_none() && xs.contains(&(pos.0+i)) {
                max_x = Some(pos.0+i-1);
            }

            if (min_x.is_some() || pos.0-i < *start) && (max_x.is_some() || pos.0+i > *end) {
                break;
            }

            i+=1;
        }

        if min_x.is_none() || max_x.is_none() {
            return None;
        }

        let (min_x, max_x) = (min_x.unwrap(), max_x.unwrap());
        // Are the walls within the floor range
        if min_x >= *floor.start() && max_x <= *floor.end() {
            return Some(min_x..=max_x);
        }

        None
    }
}

fn main() -> Result<()> {
    assert_eq!(Cavern::from("test1.input", false)?.water().len(), 57);
    assert_eq!(Cavern::from("test2.input", false)?.water().len(), 54);
    assert_eq!(Cavern::from("test3.input", false)?.water().len(), 98);

    println!("Water tile count: {}", Cavern::from("input", true)?.water().len());

    Ok(())
}
