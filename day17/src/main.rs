use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;
use std::collections::{HashMap, HashSet};
use std::collections::VecDeque;
use std::ops::RangeInclusive as RangeI;

struct Cavern {
    clay: HashMap<usize, HashSet<usize>>,
    water: HashMap<Pos, WaterType>,
    debug: bool,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum WaterType {
    Flowing,
    Settled,
}

#[derive(PartialEq)]
enum Flow {
    Down,
    Sideways,
    Up,
}

type Pos = (usize, usize);

impl fmt::Display for Flow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Flow::Down => "down",
            Flow::Up => "up",
            Flow::Sideways => "sideways",
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
            for x in min_x-2..=max_x+2 {
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

        self.trace_stream((500, min_y), Flow::Down, *max_y);

        self.water.clone()
    }

    fn trace_stream(&mut self, pos: Pos, dir: Flow, max_y: usize) {
        let mut deque = VecDeque::new();
        deque.push_back((pos, dir));

        while deque.len() > 0 {
            let (pos, dir) = deque.pop_front().unwrap();

            match dir {
                Flow::Sideways => {
                    let floor = self.floor(pos);
                    if floor.is_none() {
                        continue;
                    }

                    let floor = floor.unwrap();
                    let (start, end) = (*floor.start(), *floor.end());
                    let (mut min_x, mut max_x) = (pos.0, pos.0);
                    let xs = self.clay.get(&pos.1);

                    if self.debug {
                        println!("Flowing {} at {:?} on {:?}", dir, pos, floor);
                    }

                    while min_x-1 >= start-1 && !self.is_settled((min_x-1, pos.1), xs) {
                        min_x -= 1;
                        self.water.insert((min_x, pos.1), WaterType::Flowing);
                    }

                    while max_x+1 <= end+1 && !self.is_settled((max_x+1, pos.1), xs) {
                        max_x += 1;
                        self.water.insert((max_x, pos.1), WaterType::Flowing);
                    }

                    if min_x < start && !deque.contains(&((min_x, pos.1), Flow::Down)) {
                        deque.push_back(((min_x, pos.1), Flow::Down));
                    }
                    if max_x > end && !deque.contains(&((max_x, pos.1), Flow::Down)) {
                        deque.push_back(((max_x, pos.1), Flow::Down));
                    }
                },
                Flow::Down => {
                    let mut pos = pos;
                    while pos.1+1 <= max_y && !self.is_settled((pos.0, pos.1+1), self.clay.get(&(pos.1+1))) {
                        pos.1 += 1;
                        self.water.insert(pos, WaterType::Flowing);
                        if self.debug {
                            println!("Flowing {} at {:?}", dir, pos);
                        }
                    }

                    if pos.1 > max_y {
                        continue;
                    }

                    if !deque.contains(&(pos, Flow::Up)) {
                        deque.push_back((pos, Flow::Up));
                    }
                },
                Flow::Up => {
                    let mut pos = pos;
                    while let Some(range) = self.container(pos, self.floor(pos)) {
                        if self.debug {
                            println!("Filling container of {:?} at {:?}", range, pos);
                        }
                        range.clone().for_each(|x| {self.water.insert((x, pos.1), WaterType::Settled);});
                        pos.1 -= 1;
                    }

                    // Overflows
                    if self.debug {
                        println!("Overflowing at {:?} {:?}", pos, self.floor(pos));
                    }
                    self.water.insert(pos, WaterType::Flowing);
                    if !deque.contains(&(pos, Flow::Sideways)) {
                        deque.push_back((pos, Flow::Sideways));
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
        let y = pos.1 + 1;
        let xs = self.clay.get(&y);
        if self.is_settled((pos.0, y), xs) {
            let (mut min_x, mut max_x) = (pos.0, pos.0);

            while self.is_settled((min_x-1, y), xs) {
                min_x -= 1;
            }

            while self.is_settled((max_x+1, y), xs) {
                max_x += 1;
            }

            return Some((min_x)..=(max_x));
        }

        None
    }

    fn is_settled(&self, pos: Pos, clay: Option<&HashSet<usize>>) -> bool {
        self.water.get(&pos).map(|&w| w == WaterType::Settled).unwrap_or(false) ||
            clay.map(|c| c.contains(&pos.0)).unwrap_or(false)
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
    assert_eq!(Cavern::from("test3.input", false)?.water().values().filter(|&&v| v == WaterType::Settled).count(), 54);

    println!("Water tile count: {}", Cavern::from("input", true)?.water().len());

    Ok(())
}
