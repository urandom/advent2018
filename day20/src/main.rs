use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

#[derive(Debug)]
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
}

#[derive(Debug)]
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
}

impl Path {
    fn new() -> Self {
        Path{steps: Vec::new()}
    }

    fn from(input: &str) -> Result<Self> {
        let reader = BufReader::new(File::open(input)?);
        let mut root = Path::new();

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
                    let mut subpath = Path::new();
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
}

fn main() -> Result<()> {
    assert_eq!(Path::from("test1.input")?.max_len(), 10);
    assert_eq!(Path::from("test2.input")?.max_len(), 18);
    assert_eq!(Path::from("test3.input")?.max_len(), 23);
    assert_eq!(Path::from("test4.input")?.max_len(), 31);

    println!("# of doors: {}", Path::from("input")?.max_len());

    Ok(())
}
