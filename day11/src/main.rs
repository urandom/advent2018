use structopt::StructOpt;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::io::BufReader;
use std::cmp;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

const COLS: usize = 300;
const GRID_SIZE: usize = COLS * COLS;

fn main() -> Result<()> {
    let cli = Cli::from_args();
    let mut reader = BufReader::new(File::open(cli.path)?);

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let serial : i32 = line.trim().parse().unwrap();
    let mut grid = [0; GRID_SIZE];

    for (i, v) in grid.iter_mut().enumerate() {
        let (x, y) = idx_to_coords(i);

        *v = power(x, y, serial);
    }

    let mut max_power : (usize, usize, i32) = (0, 0, 0);
    for x in 0..COLS-2 {
        for y in 0..COLS-2 {
            if x < COLS - 2 && y < COLS - 2 {
                let p = square_power(x, y, 3, grid);
                let (_, _, max_p) = max_power;
                if p > max_p {
                    max_power = (x, y, p);
                }
            }
        }
    }

    let (max_x, max_y, _) = max_power;
    println!("Coordinates with max power for 3x3 box: {},{}", max_x, max_y);

    let mut max_power : (usize, usize, usize, i32) = (0, 0, 0, 0);
    for x in 0..COLS {
        for y in 0..COLS {
            let min = COLS - cmp::max(x, y) - 1;

            let mut p = 0;
            for s in 0..=min {
                for sy in 0..=s {
                    p += grid[(y+sy)*COLS+x+s];
                    p += grid[(y+s)*COLS+x+sy];
                }
                // Remove the duplicate
                p -= grid[(y+s)*COLS+x+s];

                let (_, _, _, max_p) = max_power;
                if p > max_p {
                    max_power = (x, y, s+1, p);
                }
            }
        }
    }

    let (max_x, max_y, max_s, _) = max_power;
    println!("Coordinates with max power for {}x{} box: {},{},{}", max_s, max_s, max_x, max_y, max_s);

    Ok(())
}

fn idx_to_coords(i: usize) -> (usize, usize) {
    let y = i / COLS;
    let x = i % COLS;

    return (x, y);
}

fn coords_to_idx(x: usize, y: usize) -> usize {
    return y * COLS + x;
}

fn square_power(x: usize, y: usize, s: usize, grid: [i32; GRID_SIZE]) -> i32 {
    let mut power : i32 = 0;
    let idx = coords_to_idx(x, y);
    for i in 0..s {
        for j in 0..s {
            power += grid[idx+j+i*COLS];
        }
    }

    return power;
}

fn power(x: usize, y: usize, serial: i32) -> i32 {
    let rack_id = (x as i32) + 10;
    let mut power = rack_id * (y as i32);

    power += serial;
    power *= rack_id;
    power = (power % 1000) / 100;

    return power - 5;
}
