use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug)]
#[derive(PartialEq)]
enum BG {
    Wall,
    Floor,
    Unit,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum UnitType {
    Goblin,
    Elf,
}

type Pos = (usize, usize);

#[derive(Debug)]
#[derive(Copy, Clone)]
struct Unit {
    kind: UnitType,
    pos: Pos,
    health: usize,
    attach: usize,
}

const AP : usize = 3;

fn main() -> Result<()> {
    let reader = BufReader::new(File::open(std::env::args().nth(1).expect("no file given"))?);
    let mut dungeon : Vec<Vec<BG>> = Vec::new();
    let mut units : Vec<Unit> = Vec::new();

    for (y, line) in reader.lines().enumerate() {
        let l = line.unwrap();

        dungeon.push(l.chars().map(|c| if c == '#' { BG::Wall } else if c == '.' { BG::Floor }  else { BG::Unit } ).collect());
        units.extend(l.chars().enumerate()
            .filter(|(_, c)| *c == 'G' || *c == 'E')
            .map(|(x, c)| Unit{
                kind: if c == 'G' { UnitType::Goblin } else { UnitType::Elf },
                pos: (x, y),
                health: 200,
                attach: AP,
            }));
    }

    print_dungeon(&dungeon, &units);
    let mut round: usize = 0;
    'turn: loop {
        units.sort_by(unit_comp);
        for i in 0..units.len() {
            let unit = units[i];
            if unit.health == 0 {
                continue;
            }

            let enemies : Vec<Pos> = units.clone().iter().filter(|u| u.kind != unit.kind).filter(|u| u.health > 0).map(|u| u.pos).collect();
            if enemies.is_empty() {
                break 'turn;
            }

            if in_range(unit.pos, &enemies).is_empty() {
                let debug = false;
                match walk(unit.pos, &enemies, &dungeon, debug) {
                    Some(pos) => {
                        let (x, y) = unit.pos;
                        dungeon[y][x] = BG::Floor;
                        units[i].pos = pos;
                        let (x, y) = units[i].pos;
                        dungeon[y][x] = BG::Unit;
                    },
                    None => (),
                }
            }
            attack(&units[i].clone(), &enemies, &mut units, &mut dungeon);
        }

        round += 1;
        units = units.iter().filter(|u| u.health > 0).map(|&u| u).collect();

        println!("After round {}", round);
        print_dungeon(&dungeon, &units);
    }

    println!("Full round: {}", round);
    print_dungeon(&dungeon, &units);

    let sum: usize = units.iter().map(|u| u.health).sum();
    println!("Outcome: {}", sum * round);

    Ok(())
}

fn attack(unit: &Unit, enemies: &Vec<Pos>, units: &mut Vec<Unit>, dungeon: &mut Vec<Vec<BG>>) -> bool {
    let ranged = in_range(unit.pos, &enemies);
    let mut ranged_enemies: Vec<Unit> = units.clone().iter().filter(|u| ranged.contains(&u.pos)).map(|&u| u).collect();
    ranged_enemies.sort_by(|a, b| {
        let l1 = a.health.cmp(&b.health);
        if l1 == Ordering::Equal {
            let mut a_idx = 0;
            let mut b_idx = 0;
            for i in 0..ranged.len() {
                if a.pos == ranged[i] {
                    a_idx = i;
                }
                if b.pos == ranged[i] {
                    b_idx = i;
                }
            }
            return a_idx.cmp(&b_idx);
        }

        return l1;
    });

    if ranged_enemies.is_empty() {
        return false;
    }

    let idx = units.iter().position(|u| u.pos == ranged_enemies[0].pos).unwrap();
    if units[idx].health < unit.attach {
        units[idx].health = 0;
        let (x, y) = units[idx].pos;
        dungeon[y][x] = BG::Floor;
    } else {
        units[idx].health -= unit.attach;
    }

    true
}

fn walk(starting: Pos, enemies: &Vec<Pos>, dungeon: &Vec<Vec<BG>>, debug: bool) -> Option<Pos> {
    let mut current: VecDeque<Pos> = VecDeque::new();
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut reverse : HashMap<Pos, Pos> = HashMap::new();
    let mut scores : Vec<Pos> = Vec::new();
    let mut min_depth = 1<<32;

    current.push_back(starting);
    'outer: while !current.is_empty() {
        let pos = current.pop_front().unwrap();

        let adj: Vec<Pos> = adjacent(pos, &visited, dungeon);
        if debug {
            println!("{:?} - {:?}", starting, adj);
        }

        for p in adj {
            reverse.insert(p, pos);
            if !in_range(p, &enemies).is_empty() {
                let (step, depth) = get_first_and_depth(&p, &starting, &reverse);
                if min_depth >= depth {
                    min_depth = depth;
                    scores.push(step);
                } else {
                    break 'outer;
                }
            }

            visited.insert(p);
            current.push_back(p);
        }
    }

    scores.sort_by_key(|s| (s.1, s.0));
    if debug {
        println!("Steps from {:?}: {:?}", starting, scores);
    }

    if scores.len() > 0 { Some(scores[0]) } else { None }
}

fn get_first_and_depth(step: &Pos, starting: &Pos, reverse: &HashMap<Pos, Pos>) -> (Pos, usize) {
    let mut key = step;
    let mut depth = 2;
    loop {
        let step = reverse.get(key);
        if step == Some(&starting) {
            return (*key, depth-1);
        } else if reverse.get(step.unwrap()) == Some(&starting) {
            return (step.map(|&p| p).unwrap(), depth);
        } else {
            key = step.unwrap();
        }
        depth += 1;
    }
}

fn in_range(pos: Pos, enemies: &Vec<Pos>) -> Vec<Pos> {
    enemies.iter().filter(|e| distance_from(&pos, e) == 1).map(|e| *e).collect()
}

fn adjacent(pos: Pos, visited: &HashSet<Pos>, dungeon: &Vec<Vec<BG>>) -> Vec<Pos> {
    let (x, y) = pos;
    let mut adjacent = Vec::new();

    if y > 0 && dungeon[y-1][x] == BG::Floor && !visited.contains(&(x, y-1)) {
        adjacent.push((x, y-1));
    }
    if x > 0 && dungeon[y][x-1] == BG::Floor && !visited.contains(&(x-1, y)) {
        adjacent.push((x-1, y));
    }
    if x+1 < dungeon[y].len() && dungeon[y][x+1] == BG::Floor && !visited.contains(&(x+1, y)) {
        adjacent.push((x+1, y));
    }
    if y+1 < dungeon.len() && dungeon[y+1][x] == BG::Floor && !visited.contains(&(x, y+1)) {
        adjacent.push((x, y+1));
    }

    return adjacent;
}

fn distance_from(a: &Pos, b: &Pos) -> usize {
    let (ax, ay) = *a;
    let (bx, by) = *b;

    ((ay as isize - by as isize).abs() + (ax as isize - bx as isize).abs()) as usize
}

fn unit_comp(a: &Unit, b: &Unit) -> Ordering {
    let (ax, ay) = a.pos;
    let (bx, by) = b.pos;

    let l1 = ay.cmp(&by);
    if l1 == Ordering::Equal {
        return ax.cmp(&bx);
    }

    return l1;
}

fn print_dungeon(dungeon: &Vec<Vec<BG>>, units: &Vec<Unit>) {
    let mut map : HashMap<Pos, Unit> = HashMap::new();
    units.iter().for_each(|u| { map.insert(u.pos, *u); });

    for (y, row) in dungeon.iter().enumerate() {
        let mut line = String::new();
        for (x, p) in row.iter().enumerate() {
            let unit = map.get(&(x, y));
            line.push(
                unit.map_or_else(
                    || if *p == BG::Wall { '#' } else if *p == BG::Unit { 'U' } else { '.' },
                    |u| if u.kind == UnitType::Elf { 'E' } else { 'G' },
                )
            );
        }

        println!("{}", line);
    }
}
