use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;

type Pos = (usize, usize);

#[derive(PartialEq, Copy, Clone, Debug)]
enum Acre {
    Open,
    Trees,
    Lumber,
}

impl Acre {
    fn mutate(self, adjacent: Vec<Acre>) -> Acre {
        match self {
            Acre::Open => {
                if adjacent.iter().filter(|&&a| a == Acre::Trees).count() >= 3 {
                    return Acre::Trees;
                }
            },
            Acre::Trees => {
                if adjacent.iter().filter(|&&a| a == Acre::Lumber).count() >= 3 {
                    return Acre::Lumber;
                }
            },
            Acre::Lumber => {
                if adjacent.iter().filter(|&&a| a == Acre::Lumber).count() == 0 ||
                    adjacent.iter().filter(|&&a| a == Acre::Trees).count() == 0 {
                    return Acre::Open;
                }
            },
        }

        self
    }
}

impl fmt::Display for Acre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Acre::Open => ".",
            Acre::Trees => "|",
            Acre::Lumber => "#",
        };
        write!(f, "{}", s)
    }
}

struct Field {
    area: Vec<Vec<Acre>>,
    debug: bool,
}

impl Field {
    fn new(debug: bool) -> Field {
        Field{area: Vec::new(), debug}
    }

    fn from(input: &str, debug: bool) -> Result<Field> {
        let reader = BufReader::new(File::open(input)?);
        let mut field = Field::new(debug);

        for line in reader.lines() {
            let line = line.unwrap();
            field.area.push(
                line.chars().map(|c| match c {
                    '.' => Acre::Open,
                    '|' => Acre::Trees,
                    '#' => Acre::Lumber,
                    _ => panic!("unknown char {}", c),
                }).collect()
            );
        }

        Ok(field)
    }

    fn iterate_for(&mut self, total: usize) -> &mut Self {
        for _ in 0..total {
            if self.debug {
                println!("{}", self);
            }
            let mut state = self.area.clone();

            for (y, row) in self.area.iter().enumerate() {
                for (x, acre) in row.iter().enumerate() {
                    state[y][x] = acre.mutate(self.adjacent_acres((x, y)));
                    
                }
            }

            self.area = state;
        }

        if self.debug {
            println!("{}", self);
        }

        self
    }

    fn resource(&self) -> usize {
        self.area.iter().flatten().filter(|&&a| a == Acre::Lumber).count() *
            self.area.iter().flatten().filter(|&&a| a == Acre::Trees).count()
    }

    fn adjacent_acres(&self, pos: Pos) -> Vec<Acre> {
        let mut adj = Vec::new();

        if pos.1 > 0 {
            if pos.0 > 0 {
                adj.push(self.area[pos.1-1][pos.0-1]);
            }
            adj.push(self.area[pos.1-1][pos.0]);
            if pos.0 < self.area.len() - 1 {
                adj.push(self.area[pos.1-1][pos.0+1]);
            }
        }
        if pos.0 > 0 {
            adj.push(self.area[pos.1][pos.0-1]);
        }
        if pos.0 < self.area.len() - 1 {
            adj.push(self.area[pos.1][pos.0+1]);
        }

        if pos.1 < self.area.len()-1 {
            if pos.0 > 0 {
                adj.push(self.area[pos.1+1][pos.0-1]);
            }
            adj.push(self.area[pos.1+1][pos.0]);
            if pos.0 < self.area.len() - 1 {
                adj.push(self.area[pos.1+1][pos.0+1]);
            }
        }

        adj
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in &self.area {
            for x in y {
                write!(f, "{}", x)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    assert_eq!(Field::from("test1.input", false)?.iterate_for(10).resource(), 1147);
    println!("Resource value: {}", Field::from("input", true)?.iterate_for(10).resource());
    Ok(())
}
