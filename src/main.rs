use std::{io::{self, Read}, char};

#[derive(PartialEq, Debug)]
enum Instruction {
    MoveRight,
    MoveLeft,
    Inc,
    Dec,
    Output,
    Input,
    LoopStart,
    LoopEnd
}

impl From<char> for Instruction {
    fn from(c: char) -> Instruction {
        match c {
            '>' => Instruction::MoveRight,
            '<' => Instruction::MoveLeft,
            '+' => Instruction::Inc,
            '-' => Instruction::Dec,
            '.' => Instruction::Output,
            ',' => Instruction::Input,
            '[' => Instruction::LoopStart,
            ']' => Instruction::LoopEnd,
            _ => panic!("Unknown instruction: {}", c)
        }
    }
}

#[derive(Debug)]
struct State {
    memory: [u8; 30_000],
    pointer_idx: usize,
    loop_starts: Vec<usize>
}

impl State {
    fn new() -> State {
        State {
            memory: [0; 30_000],
            pointer_idx: 0,
            loop_starts: vec![]
        }
    }
}

fn find_matching_loop_end_pos(curr_instr_pointer: usize, instructions: &Vec<Instruction>) -> usize {
    let mut idx = curr_instr_pointer + 1;
    let mut opened_loops_inside = 0;
    if idx >= instructions.len() {
        panic!("No matching loop end!");
    }
    while 
        instructions[idx] != Instruction::LoopEnd 
        || opened_loops_inside > 0 {
            if instructions[idx] == Instruction::LoopStart {
                opened_loops_inside += 1;
            } else if instructions[idx] == Instruction::LoopEnd {
                opened_loops_inside -= 1;
            }

            idx += 1;
            if idx >= instructions.len() {
                panic!("No matching loop end!");
            }
    }
    
    return idx;
}

// returns next instruction pointer
fn interpret_one(state: &mut State, instructions: &Vec<Instruction>, instr_pointer: usize) -> usize {
    match instructions[instr_pointer] {
        Instruction::MoveRight => { 
            state.pointer_idx += 1;
            instr_pointer + 1
        },
        Instruction::MoveLeft => {
            state.pointer_idx -= 1;
            instr_pointer + 1
        },
        Instruction::Inc => {
            state.memory[state.pointer_idx] += 1;
            instr_pointer + 1
        },
        Instruction::Dec => { 
            state.memory[state.pointer_idx] -= 1;
            instr_pointer + 1
        },
        Instruction::Output => { 
            print!("{}", char::from(state.memory[state.pointer_idx]));
            instr_pointer + 1
        },
        Instruction::Input => {
            let x = io::stdin().bytes().next().expect("Error while reading input.").unwrap();
            state.memory[state.pointer_idx] = x;
            instr_pointer + 1
        },
        Instruction::LoopStart => {
            let x = state.memory[state.pointer_idx];
            if x > 0 {
                state.loop_starts.push(instr_pointer);
                instr_pointer + 1
            } else {
                let idx = find_matching_loop_end_pos(instr_pointer, instructions);
                idx + 1
            }
        },
        Instruction::LoopEnd => {
            state.loop_starts.pop().expect("Error, no matching loop start.")
        },
    }
}

fn interpret_many(instructions: Vec<Instruction>) {
    let mut state = State::new();
    let mut instr_pointer: usize = 0;
    let instructions_len = instructions.len();

    while instr_pointer < instructions_len {
        instr_pointer = interpret_one(&mut state, &instructions, instr_pointer)
    }
}

fn get_input() -> String {
    return "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.".to_string()
}

fn main() {
    let instructions: Vec<Instruction> = get_input()
        .as_bytes()
        .into_iter()
        .map(|x| char::from(*x).into())
        .collect();

    interpret_many(instructions);
}
