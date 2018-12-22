use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

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
}

fn main() -> Result<()> {
    assert_eq!(Cave::from("test1.input")?.risk_to_target(), 114);

    println!("Risk to target: {}", Cave::from("input")?.risk_to_target());
    Ok(())
}
