use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;

type Data = [usize; 4];
type Operands = [usize; 3];

#[derive(Debug, Copy, Clone)]
struct TestData(Data, Data, Data);

#[derive(Debug)]
struct Device {
    registers: Data,

    testing: Vec<TestData>,
    instructions: Vec<Data>,
    opcodes: Vec<OpCode>,
}

#[derive(Debug)]
enum Oper {
    Addr, Addi,
    Mulr, Muli,
    Banr, Bani,
    Borr, Bori,
    Setr, Seti,
    Gtir, Gtri, Gtrr,
    Eqir, Eqri, Eqrr,
}

#[derive(Debug)]
struct OpCode {
    number: usize,
    operation: Oper,
    operands: Operands,
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

    fn num_samples_for_opcodes(&mut self, size: usize) -> usize {
        let mut count = 0;
        for test in &self.testing {
            let mut matching = 0;

            for op in &mut self.opcodes {
                let mut opcode = op;
                opcode.number = test.1[0];
                opcode.operands = data_to_operands(test.1);
                let mut registers = test.0;

                opcode.call(&mut registers);

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
}

impl OpCode {
    fn new(operation: Oper) -> OpCode {
        OpCode{number: 0, operation, operands: [0; 3]}
    }

    fn generate() -> Vec<OpCode> {
        vec![
            OpCode::new(Oper::Addr), OpCode::new(Oper::Addi),
            OpCode::new(Oper::Mulr), OpCode::new(Oper::Muli),
            OpCode::new(Oper::Banr), OpCode::new(Oper::Bani),
            OpCode::new(Oper::Borr), OpCode::new(Oper::Bori),
            OpCode::new(Oper::Setr), OpCode::new(Oper::Seti),
            OpCode::new(Oper::Gtir), OpCode::new(Oper::Gtri), OpCode::new(Oper::Gtrr),
            OpCode::new(Oper::Eqir), OpCode::new(Oper::Eqri), OpCode::new(Oper::Eqrr),
        ]
    }

    fn call(&self, registers: &mut Data) {
        match self.operation {
            Oper::Addr => registers[self.operands[2]] =
                registers[self.operands[0]] + registers[self.operands[1]],
            Oper::Addi => registers[self.operands[2]] =
                registers[self.operands[0]] + self.operands[1],
            Oper::Mulr => registers[self.operands[2]] =
                registers[self.operands[0]] * registers[self.operands[1]],
            Oper::Muli => registers[self.operands[2]] =
                registers[self.operands[0]] * self.operands[1],
            Oper::Banr => registers[self.operands[2]] =
                registers[self.operands[0]] & registers[self.operands[1]],
            Oper::Bani => registers[self.operands[2]] =
                registers[self.operands[0]] & self.operands[1],
            Oper::Borr => registers[self.operands[2]] =
                registers[self.operands[0]] | registers[self.operands[1]],
            Oper::Bori => registers[self.operands[2]] =
                registers[self.operands[0]] | self.operands[1],
            Oper::Setr => registers[self.operands[2]] = registers[self.operands[0]],
            Oper::Seti => registers[self.operands[2]] = self.operands[0],
            Oper::Gtir => registers[self.operands[2]] =
                if self.operands[0] > registers[self.operands[1]] { 1 } else { 0 },
            Oper::Gtri => registers[self.operands[2]] =
                if registers[self.operands[0]] > self.operands[1] { 1 } else { 0 },
            Oper::Gtrr => registers[self.operands[2]] =
                if registers[self.operands[0]] > registers[self.operands[1]] { 1 } else { 0 },
            Oper::Eqir => registers[self.operands[2]] =
                if self.operands[0] == registers[self.operands[1]] { 1 } else { 0 },
            Oper::Eqri => registers[self.operands[2]] =
                if registers[self.operands[0]] == self.operands[1] { 1 } else { 0 },
            Oper::Eqrr => registers[self.operands[2]] =
                if registers[self.operands[0]] == registers[self.operands[1]] { 1 } else { 0 },
        }
    }
}

fn main() -> Result<()> {
    assert_eq!(Device::from("test1.input")?.num_samples_for_opcodes(3), 1);
    println!("Sample count for 3 or more opcodes: {}", Device::from("input")?.num_samples_for_opcodes(3));
    
    Ok(())
}

fn vec_to_data(d: &Vec<usize>) -> Data {
    let mut data = [0; 4];

    for i in 0..4 {
        data[i] = d[i];
    }

    data
}

fn data_to_operands(d: Data) -> Operands {
    [d[1], d[2], d[3]]
}
