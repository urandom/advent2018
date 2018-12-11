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
    for (i, _) in grid.iter().enumerate() {
        let (x, y) = idx_to_coords(i);
        if x < COLS - 2 && y < COLS - 2 {
            let p = square_power(x, y, grid);
            let (_, _, max_p) = max_power;
            if p > max_p {
                max_power = (x, y, p);
            }
        }
    }

    let (max_x, max_y, _) = max_power;
    println!("Coordinates with max power: {},{}", max_x, max_y);

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

fn square_power(x: usize, y: usize, grid: [i32; GRID_SIZE]) -> i32 {
    let i1 = coords_to_idx(x, y);
    let mut power = grid[i1];
    power += grid[i1+1];
    power += grid[i1+2];

    let i2 = coords_to_idx(x, y+1);
    power += grid[i2];
    power += grid[i2+1];
    power += grid[i2+2];

    let i3 = coords_to_idx(x, y+2);
    power += grid[i3];
    power += grid[i3+1];
    power += grid[i3+2];

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
