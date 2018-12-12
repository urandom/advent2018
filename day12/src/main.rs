use structopt::StructOpt;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::io::BufReader;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct Rule {
    pattern: Vec<bool>,
    result: bool,
}

const GEN : usize = 20;

fn main() -> Result<()> {
    let cli = Cli::from_args();
    let mut reader = BufReader::new(File::open(cli.path)?);

    let mut initial = String::new();
    reader.read_line(&mut initial)?;
    initial = initial.split_off(15);

    let mut offset = 4;
    let mut state : Vec<bool> = Vec::new();
    for char in initial.chars() {
        state.push(char == '#');
    }
    (0..4).for_each(|_| state.insert(0, false));
    (0..4).for_each(|_| state.push(false));

    initial.clear();
    reader.read_line(&mut initial)?;

    let mut rules : Vec<Rule> = Vec::new();
    for line in reader.lines() {
        let line_str = line.unwrap();
        let mut r = Rule{pattern: Vec::new(), result: false};
        let mut iter = line_str.split(" => ");
        let pat = iter.next();
        let res = iter.next();

        for char in pat.unwrap().chars() {
            r.pattern.push(char == '#');
        }
        r.result = res.unwrap() == "#";

        rules.push(r);
    }

    for _ in 0..GEN {
        let orig = state.to_vec();
        let mut left = 0;
        let mut right = 0;
        for i in 2..state.len()-2 {
            for r in &rules {
                if r.pattern == &orig[i-2..=i+2] {
                    state[i] = r.result;
                    if r.result {
                        if i < offset {
                            left += 1;
                        } else if i > state.len() - offset {
                            right += 1;
                        }
                    }
                }
            }
        }
        (0..left).for_each(|_| state.insert(0, false));
        (0..right).for_each(|_| state.push(false));
        offset += left;
    }

    println!("Sum: {}", calc_potted(&state, offset));

    Ok(())
}

fn calc_potted(state: &Vec<bool>, offset: usize) -> i32 {
    state.iter().enumerate().filter(|&(_, v)| *v).map(|(i, _)| i as i32).map(|i| i - (offset as i32)).sum()
}

fn print_state(state: &Vec<bool>) {
    let line = state.iter().map(move |v| if *v { '#' } else { '.' }).collect::<String>();
    println!("{}", line);
}
