use std::fs;

const OPCODE_ADD: i32 = 1;
const OPCODE_MULT: i32 = 2;
const OPCODE_QUIT: i32 = 99;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let mut program: Vec<i32> = contents
        .split(",")
        .map(|str| str.parse::<i32>().unwrap())
        .collect();

    program[1] = 12;
    program[2] = 2;

    let mut pc = 0;
    while pc < program.len() {
        let op = program[pc];
        if op == OPCODE_QUIT {
            break;
        } else {
            let pa = program[pc + 1] as usize;
            let pb = program[pc + 2] as usize;
            let pout = program[pc + 3] as usize;

            let result = match op {
                OPCODE_ADD => program[pa] + program[pb],
                OPCODE_MULT => program[pa] * program[pb],
                _ => 0,
            };

            program[pout] = result;
        }
        pc += 4;
    }

    println!("Program execution finished");
    println!("Final program state:\n{:?}", program);
}
