use std::fmt;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;
use std::iter::FromIterator;

type Registers = [usize; 6];
type Operands = [usize; 3];
type Instructions = Vec<(OpCode, Operands)>;

#[derive(Debug)]
struct Device {
    pointer: usize,
    registers: Registers,
    instructions: Instructions,
    debug: bool,
}

#[derive(Debug, Copy, Clone)]
enum Oper {
    Addr, Addi,
    Mulr, Muli,
    Banr, Bani,
    Borr, Bori,
    Setr, Seti,
    Gtir, Gtri, Gtrr,
    Eqir, Eqri, Eqrr,
}

#[derive(Debug, Copy, Clone)]
struct OpCode {
    operation: Oper,
}

impl Device {
    fn new(pointer: usize, instructions: Instructions, debug: bool) -> Device {
        Device{pointer, registers: [0; 6], instructions, debug}
    }

    fn from(input: &str, debug: bool) -> Result<Device> {
        let reader = BufReader::new(File::open(input)?);
        let mut pointer: usize = 0;
        let opcodes = OpCode::generate();
        let mut instructions = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            if line.starts_with("#ip ") {
                pointer = line[4..].parse().unwrap();
                continue;
            }

            let mut v: Vec<&str> = line.split(' ').collect();
            let opcode = opcodes[v.remove(0)];
            let data: Vec<usize> = v.iter().map(|r| r.parse().unwrap()).collect();

            instructions.push((opcode, [data[0], data[1], data[2]]));
        }

        Ok(Device::new(pointer, instructions, debug))
    }

    fn execute(&mut self) -> &mut Self {
        while self.registers[self.pointer] + 1 < self.instructions.len() {
            if self.debug {
                println!("Before {:?}", self.registers);
            }

            self.execute1();

            if self.debug {
                println!("After {:?}", self.registers);
            }

        }

        self
    }

    fn optimized(&mut self) -> &mut Self {
        let mut stable = self.registers[1];
        let mut count = 0;
        loop {
            self.execute1();
            if self.registers[1] == stable {
                if count > 10 {
                    break;
                }
                count += 1;
            } else {
                count = 0;
            }
            stable = self.registers[1];
        }

        let mut multiples = HashSet::new();
        for i in 1..stable {
            if stable % i == 0 {
                let other = stable / i;
                if multiples.contains(&other) {
                    break;
                }
                multiples.insert(i);
                multiples.insert(stable / i);
            }
        }

        if self.debug {
            println!("Multiples of {}: {:?}", stable, multiples);
        }

        self.registers[0] = multiples.iter().sum();

        self
    }

    fn execute1(&mut self) {
        let instruction = self.instructions[self.registers[self.pointer]];
        let op = instruction.0;

        self.registers = op.call(&self.registers, instruction.1);
        self.registers[self.pointer] += 1;
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl OpCode {
    fn new(operation: Oper) -> OpCode {
        OpCode{operation}
    }

    fn generate() -> HashMap<&'static str, OpCode> {
        HashMap::from_iter(
            vec![
                OpCode::new(Oper::Addr), OpCode::new(Oper::Addi),
                OpCode::new(Oper::Mulr), OpCode::new(Oper::Muli),
                OpCode::new(Oper::Banr), OpCode::new(Oper::Bani),
                OpCode::new(Oper::Borr), OpCode::new(Oper::Bori),
                OpCode::new(Oper::Setr), OpCode::new(Oper::Seti),
                OpCode::new(Oper::Gtir), OpCode::new(Oper::Gtri), OpCode::new(Oper::Gtrr),
                OpCode::new(Oper::Eqir), OpCode::new(Oper::Eqri), OpCode::new(Oper::Eqrr),
            ].iter().map(|op| (op.name(), *op))
        )
    }

    fn name(&self) -> &'static str {
        match self.operation {
            Oper::Addr => "addr",
            Oper::Addi => "addi",
            Oper::Mulr => "mulr",
            Oper::Muli => "muli",
            Oper::Banr => "banr",
            Oper::Bani => "bani",
            Oper::Borr => "borr",
            Oper::Bori => "bori",
            Oper::Setr => "setr",
            Oper::Seti => "seti",
            Oper::Gtir => "gtir",
            Oper::Gtri => "gtri",
            Oper::Gtrr => "gtrr",
            Oper::Eqir => "eqir",
            Oper::Eqri => "eqri",
            Oper::Eqrr => "eqrr",
        }
    }

    fn call(&self, original: &Registers, operands: Operands) -> Registers {
        let mut registers = original.clone();
        match self.operation {
            Oper::Addr => registers[operands[2]] =
                registers[operands[0]] + registers[operands[1]],
            Oper::Addi => registers[operands[2]] =
                registers[operands[0]] + operands[1],
            Oper::Mulr => registers[operands[2]] =
                registers[operands[0]] * registers[operands[1]],
            Oper::Muli => registers[operands[2]] =
                registers[operands[0]] * operands[1],
            Oper::Banr => registers[operands[2]] =
                registers[operands[0]] & registers[operands[1]],
            Oper::Bani => registers[operands[2]] =
                registers[operands[0]] & operands[1],
            Oper::Borr => registers[operands[2]] =
                registers[operands[0]] | registers[operands[1]],
            Oper::Bori => registers[operands[2]] =
                registers[operands[0]] | operands[1],
            Oper::Setr => registers[operands[2]] = registers[operands[0]],
            Oper::Seti => registers[operands[2]] = operands[0],
            Oper::Gtir => registers[operands[2]] =
                if operands[0] > registers[operands[1]] { 1 } else { 0 },
            Oper::Gtri => registers[operands[2]] =
                if registers[operands[0]] > operands[1] { 1 } else { 0 },
            Oper::Gtrr => registers[operands[2]] =
                if registers[operands[0]] > registers[operands[1]] { 1 } else { 0 },
            Oper::Eqir => registers[operands[2]] =
                if operands[0] == registers[operands[1]] { 1 } else { 0 },
            Oper::Eqri => registers[operands[2]] =
                if registers[operands[0]] == operands[1] { 1 } else { 0 },
            Oper::Eqrr => registers[operands[2]] =
                if registers[operands[0]] == registers[operands[1]] { 1 } else {
                    println!("{} - {} - {} - {}", operands[0], operands[1], registers[operands[0]], registers[operands[1]]);
                    0 
                },
        }

        registers
    }
}

fn main() -> Result<()> {
    let mut device = Device::from("input", true)?;
    device.registers[0] = 212115;
    println!("Value of registers: {:?}", device.execute().registers);

    Ok(())
}
