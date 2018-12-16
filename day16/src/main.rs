use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;
use std::iter::FromIterator;

type Data = [usize; 4];
type Operands = [usize; 3];

#[derive(Debug, Copy, Clone)]
struct TestData(Data, Data, Data);

#[derive(Debug)]
struct Device {
    registers: Data,

    testing: Vec<TestData>,
    instructions: Vec<Data>,
    opcodes: HashMap<i32, OpCode>,
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
    number: i32,
    operation: Oper,
}

impl TestData {
    fn new() -> TestData {
        TestData([0; 4], [0; 4], [0; 4])
    }
}

impl Device {
    fn new() -> Device {
        Device{registers: [0; 4], testing: Vec::new(), instructions: Vec::new(), opcodes: OpCode::generate()}
    }

    fn from(input: &str) -> Result<Device> {
        let reader = BufReader::new(File::open(input)?);
        let mut device = Device::new();

        let mut test_data = TestData::new();
        let mut want_test_data = false;

        for line in reader.lines() {
            let mut line = line.unwrap();
            if line.is_empty() {
                continue;
            }

            if line.starts_with("Before: [") {
                let line = line.split_off(9);

                let data: Vec<usize> = line.split(", ").map(|s| s.trim_end_matches(']').parse().unwrap()).collect();
                test_data.0 = vec_to_data(&data);

                want_test_data = true;
            } else if want_test_data {
                if line.starts_with("After:  [") {
                    let line = line.split_off(9);
                    let data: Vec<usize> = line.split(", ").map(|s| s.trim_end_matches(']').parse().unwrap()).collect();
                    test_data.2 = vec_to_data(&data);

                    want_test_data = false;
                    device.testing.push(test_data);
                    test_data = TestData::new();
                } else {
                    let data: Vec<usize> = line.split(" ").map(|s| s.parse().unwrap()).collect();
                    test_data.1 = vec_to_data(&data);
                }
            } else {
                let data: Vec<usize> = line.split(" ").map(|s| s.parse().unwrap()).collect();
                device.instructions.push(vec_to_data(&data));
            }
        }

        Ok(device)
    }

    fn num_samples_for_opcodes(&self, size: usize) -> usize {
        let mut count = 0;
        for test in &self.testing {
            let mut matching = 0;

            for (_, op) in &self.opcodes {
                let mut opcode = op.clone();
                let (num, operands) = data_to_operands(test.1);
                opcode.number = num;

                let registers = opcode.call(&test.0, operands);

                if test.2 == registers {
                    matching += 1;
                }
            }

            if matching >= size {
                count += 1;
            }
        }

        count
    }

    fn identify_opcodes(&mut self) {
        let mut all_matching = HashMap::new();
        let mut found_ids = HashSet::new();

        for test in &self.testing {
            if found_ids.contains(&test.1[0]) {
                continue;
            }

            let mut matching = Vec::new();

            for (_, op) in &self.opcodes {
                if op.is_valid() {
                    continue;
                }

                let mut opcode = op.clone();
                let (num, operands) = data_to_operands(test.1);
                opcode.number = num;

                let registers = opcode.call(&test.0, operands);

                if test.2 == registers {
                    matching.push(op.number);
                }
            }

            if matching.len() == 1 {
                let mut op = self.opcodes.remove(&matching[0]).unwrap();
                found_ids.insert(test.1[0]);

                op.number = test.1[0] as i32;
                self.opcodes.insert(op.number, op);

                all_matching.remove(&test.1[0]);
            } else {
                all_matching.insert(test.1[0], matching);
            }
        }
    }

    fn execute_sample(&self) -> usize {
        let mut registers = self.registers;
        for instr in &self.instructions {
            let (num, operands) = data_to_operands(*instr);
            let op = self.opcodes.get(&num).unwrap();
            registers = op.call(&registers, operands);
        }

        registers[0]
    }
}

impl OpCode {
    fn new(number: i32, operation: Oper) -> OpCode {
        OpCode{number, operation}
    }

    fn generate() -> HashMap<i32, OpCode> {
        HashMap::from_iter(
            vec![
                OpCode::new(-1,Oper::Addr), OpCode::new(-2, Oper::Addi),
                OpCode::new(-3, Oper::Mulr), OpCode::new(-4, Oper::Muli),
                OpCode::new(-5, Oper::Banr), OpCode::new(-6, Oper::Bani),
                OpCode::new(-7, Oper::Borr), OpCode::new(-8, Oper::Bori),
                OpCode::new(-9, Oper::Setr), OpCode::new(-10, Oper::Seti),
                OpCode::new(-11, Oper::Gtir), OpCode::new(-12, Oper::Gtri), OpCode::new(-13, Oper::Gtrr),
                OpCode::new(-14, Oper::Eqir), OpCode::new(-15, Oper::Eqri), OpCode::new(-16, Oper::Eqrr),
            ].iter().map(|op| (op.number, *op))
        )
    }


    fn call(&self, original: &Data, operands: Operands) -> Data {
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
                if registers[operands[0]] == registers[operands[1]] { 1 } else { 0 },
        }

        registers
    }

    fn is_valid(&self) -> bool {
        self.number >= 0
    }
}

fn main() -> Result<()> {
    assert_eq!(Device::from("test1.input")?.num_samples_for_opcodes(3), 1);

    let mut device = Device::from("input")?;
    let samples = device.num_samples_for_opcodes(3);
    assert_eq!(samples, 547);
    println!("Sample count for 3 or more opcodes: {}", samples);

    device.identify_opcodes();
    let sample = device.execute_sample();
    assert_eq!(sample, 582);
    println!("Register 0: {}", device.execute_sample());
    
    Ok(())
}

fn vec_to_data(d: &Vec<usize>) -> Data {
    let mut data = [0; 4];

    for i in 0..4 {
        data[i] = d[i];
    }

    data
}

fn data_to_operands(d: Data) -> (i32, Operands) {
    (d[0] as i32, [d[1], d[2], d[3]])
}
