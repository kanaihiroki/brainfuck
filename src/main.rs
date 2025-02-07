use std::{
    env::args, error::Error, io::{BufReader, Read, Result as IoResult}, process
};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    Output,
    Input,
    LoopStart(usize),
    LoopEnd(usize),
}

fn parse<Reader: Read>(reader: Reader) -> IoResult<Vec<Instruction>> {
    // read input file char by char
    let mut code: Vec<Instruction> = Vec::new();
    let mut loop_stack: Vec<usize> = Vec::new();

    for byte in reader.bytes() {
        match byte? {
            b'>' => code.push(Instruction::IncrementPointer),
            b'<' => code.push(Instruction::DecrementPointer),
            b'+' => code.push(Instruction::IncrementValue),
            b'-' => code.push(Instruction::DecrementValue),
            b'.' => code.push(Instruction::Output),
            b',' => code.push(Instruction::Input),
            b'[' => {
                loop_stack.push(code.len());
                code.push(Instruction::LoopStart(0));
            }
            b']' => {
                let start = loop_stack.pop().unwrap();
                code[start] = Instruction::LoopStart(code.len());
                code.push(Instruction::LoopEnd(start));
            }
            _ => {}
        }
    }

    return Ok(code);
}

fn interpret(code: &[Instruction]) -> () {
    let mut ip = 0;
    let mut dp = 0;
    let mut data = [0u8; 30000];

    while ip < code.len() {
        match code[ip] {
            Instruction::IncrementPointer => dp += 1,
            Instruction::DecrementPointer => dp -= 1,
            Instruction::IncrementValue => data[dp] = data[dp].wrapping_add(1),
            Instruction::DecrementValue => data[dp] = data[dp].wrapping_sub(1),
            Instruction::Output => print!("{}", data[dp] as char),
            Instruction::Input => {
                let mut buffer = [0u8; 1];
                std::io::stdin().read_exact(&mut buffer).unwrap();
                data[dp] = buffer[0];
            }
            Instruction::LoopStart(end) => {
                if data[dp] == 0 {
                    ip = end;
                }
            }
            Instruction::LoopEnd(start) => {
                if data[dp] != 0 {
                    ip = start;
                }
            }
        }

        ip += 1;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    match args().collect::<Vec<String>>().as_slice() {
        [_, input_file] => {
            let fd = std::fs::File::open(input_file)?;
            let reader = BufReader::new(fd);
            let code = parse(reader)?;
            interpret(&code);
        }
        _ => {
            eprintln!("Usage: brainfuck <input>");
            process::exit(1);
        }
    }

    Ok(())
}
