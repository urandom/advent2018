use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;
use std::collections::HashSet;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq)]
enum Turn {
    Left,
    Right,
    Straight,
}

const INTER : [Turn; 3] = [Turn::Left, Turn::Straight, Turn::Right];

#[derive(Debug)]
struct Cart {
    x: usize,
    y: usize,
    dir: Dir,
    inter: usize,
}

fn main() -> Result<()> {
    let reader = BufReader::new(File::open(std::env::args().nth(1).unwrap())?);
    let mut tracks : Vec<Vec<char>> = Vec::new();
    let mut carts : Vec<Cart> = Vec::new();

    for (y, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        let chars = l.chars();
        tracks.push(chars.collect());

        let c : Vec<Cart> =
            tracks.last().unwrap().iter().enumerate()
                .filter(|(_, &v)| v == '<' || v == '>' || v == '^' || v == 'v')
                .map(|(x, &v)| match v {
                    '>' => (x, Dir::Right), 
                    '<' => (x, Dir::Left), 
                    '^' => (x, Dir::Up), 
                    'v' => (x, Dir::Down), 
                    _ => panic!("unknown char"),
                })
                .map(|(x, d)| Cart{x: x, y: y, dir: d, inter: 0})
                .collect();

        for cart in &c {
            tracks[cart.y][cart.x] = match cart.dir {
                Dir::Right => '-',
                Dir::Left => '-',
                Dir::Up => '|',
                Dir::Down => '|',
            }
        }

        carts.extend(c);
    }

    let mut positions : HashSet<(usize, usize)> = HashSet::new();
    'outer: loop {
        carts.sort_by(cart_comp);

        for c in &mut carts {
            positions.remove(&(c.x, c.y));
            match c.dir {
                Dir::Right => c.x += 1,
                Dir::Left => c.x -= 1,
                Dir::Up => c.y -= 1,
                Dir::Down => c.y += 1,
            };
            track_to_dir(tracks[c.y][c.x], &mut c.dir, &mut c.inter);

            if positions.contains(&(c.x, c.y)) {
                println!("Collision: {},{}", c.x, c.y);
                break 'outer;
            }
            positions.insert((c.x, c.y));
        }
    }

    Ok(())
}

fn track_to_dir(t: char, d: &mut Dir, i: &mut usize) {
    match t {
        '|' => (), 
        '-' => (),
        '/' => {
            *d = match d {
                Dir::Up => Dir::Right,
                Dir::Right => Dir::Up,
                Dir::Left => Dir::Down,
                _ => Dir::Left,
            }
        }, 
        '\\' => {
            *d = match d {
                Dir::Up => Dir::Left,
                Dir::Left => Dir::Up,
                Dir::Right => Dir::Down,
                _ => Dir::Right,
            }
        },
        '+' => {
            *d = match INTER[*i % INTER.len()] {
                Turn::Left => match d {
                    Dir::Down => Dir::Right,
                    Dir::Up => Dir::Left,
                    Dir::Left => Dir::Down,
                    Dir::Right => Dir::Up,
                },
                Turn::Right => match d {
                    Dir::Down => Dir::Left,
                    Dir::Up => Dir::Right,
                    Dir::Left => Dir::Up,
                    Dir::Right => Dir::Down,
                },
                Turn::Straight => *d,
            };

            *i += 1;
        }, 
        _ => panic!("unknown char '{}'", t),
    };
}

fn cart_comp(a: &Cart, b: &Cart) -> Ordering {
    let l1 = a.y.cmp(&b.y);
    if l1 == Ordering::Equal {
        return a.x.cmp(&b.x);
    }

    return l1;
}
