use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;

fn main() -> Result<()> {
    let mut reader = BufReader::new(File::open(std::env::args().nth(1).unwrap())?);
    let mut line = String::new();

    reader.read_line(&mut line)?;
    let upto : usize = line.trim().parse().unwrap();
    let total = upto + 10;
    let mut current = [0, 1];
    let mut scores = vec![3, 7];
    let digits = digits(upto);
    let digits = digits.as_slice();

    loop {//while scores.len() < total {
        let sum = scores[current[0]] + scores[current[1]];
        if sum > 9 {
            scores.push(sum / 10);

            if scores.len() == total {
                //break;
            }
        }
        scores.push(sum % 10);

        current[0] = (current[0] + scores[current[0]] + 1) % scores.len();
        current[1] = (current[1] + scores[current[1]] + 1) % scores.len();

        if scores.len() >= digits.len()+1 {
            if scores[scores.len() - digits.len()..] == *digits {
                println!("Number of recipes: {}", scores.len() - digits.len());
                break;
            }
            if scores[scores.len() - digits.len() - 1..scores.len() - 1] == *digits {
                println!("Number of recipes: {}", scores.len() - digits.len()-1);
                break;
            }
        }
    }

    let mut res = String::new();
    for i in scores.len()-10..scores.len() {
        res.push_str((scores[i] as u32).to_string().as_str());
    }
    println!("Last 10 scores: {}", res);

    Ok(())
}

fn digits(upto: usize) -> Vec<usize> {
    let mut num = upto;
    let mut digits = Vec::new();

    while num != 0 {
        let r = num % 10;
        digits.insert(0, r);
        num /= 10;
    }

    return digits;
}
