use std::fs::File;
use std::io::{BufRead, BufReader, Result};

type Pos = (i32, i32, i32);

trait Sizer {
    fn dist_to(&self, other: &Self) -> usize;
}

impl Sizer for Pos {
    fn dist_to(&self, other: &Pos) -> usize {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs() + (self.2 - other.2).abs()) as usize
    }
}

#[derive(Debug)]
struct Bot {
    pos: Pos,
    radius: usize,
}

impl Bot {
    fn new(coords: Vec<i32>, radius: usize) -> Bot {
        Bot{pos: (coords[0], coords[1], coords[2]), radius}
    }

    fn dist_to(&self, target: &Self) -> usize {
        self.pos.dist_to(&target.pos)
    }
}

#[derive(Debug)]
struct Network {
    bots: Vec<Bot>
}

impl Network {
    fn new(bots: Vec<Bot>) -> Network {
        Network{bots}
    }

    fn from(input: &str) -> Result<Network> {
        let reader = BufReader::new(File::open(input)?);
        let mut bots = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            let line = &line[5..];
            let mut iter = line.splitn(2, ">, r=");
            let coords: Vec<i32> = iter.next().unwrap().split(",").map(|v| v.parse::<i32>().expect(v)).collect();
            let rad: usize = iter.next().unwrap().parse::<usize>().unwrap();

            bots.push(Bot::new(coords, rad));
        }

        Ok(Network::new(bots))
    }

    fn strongest(&self) -> &Bot {
        let mut strongest = &self.bots[0];
        for bot in &self.bots {
            if bot.radius > strongest.radius {
                strongest = bot;
            }
        }

        strongest
    }

    fn in_range_of_strongest(&self) -> Vec<&Bot> {
        self.in_range_of(self.strongest())
    }

    fn in_range_of(&self, target: &Bot) -> Vec<&Bot> {
        self.bots.iter().filter(|&b| b.dist_to(target) <= target.radius).collect()
    }
}

fn main() -> Result<()> {
    assert_eq!(Network::from("test1.input")?.in_range_of_strongest().len(), 7);

    println!("In range of strongest: {}", Network::from("input")?.in_range_of_strongest().len());
    Ok(())
}
