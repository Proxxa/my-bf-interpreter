
const LEFT_CH : char = '<';
const RIGHT_CH : char = '>';
const INC_CH : char = '+';
const DEC_CH : char = '-';
const IN_CH : char = ',';
const OUT_CH : char = '.';
const LOOP_CH : char = '[';
const ELOOP_CH : char = ']';

#[allow(dead_code)]
#[derive(Debug)]
pub struct BFProgram {
    pub instructions: Vec<Instruction>,
}

// the usize for LOOP and ELOOP are the associated instructions. DUMMY is used before an ELOOP is matched to a LOOP.
#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    LEFT,
    RIGHT,
    INC,
    DEC,
    IN,
    OUT,
    LOOP(usize),
    ELOOP(usize),
    DUMMY,
}

pub fn compile(instructions: String) -> BFProgram {
    use Instruction::*;

    validate(instructions.clone());

    let mut invec : Vec<Instruction> = vec![];
    let mut nesting : Vec<usize> = vec![];
    let mut instruction = 0;
    let mut a: usize;

    for c in instructions.as_bytes() {
        match *c as char {
            LEFT_CH => invec.push(LEFT),
            RIGHT_CH => invec.push(RIGHT),
            INC_CH => invec.push(INC),
            DEC_CH => invec.push(DEC),
            IN_CH => invec.push(IN),
            OUT_CH => invec.push(OUT),
            LOOP_CH => {
                invec.push(DUMMY);
                nesting.push(instruction);
            },
            ELOOP_CH => {
                a = nesting.pop().expect("Somehow validated invalid instructions.");
                invec[a] = LOOP(instruction);
                invec.push(ELOOP(a));
            },
            _ => continue, // skip increment
        }

        instruction += 1;
    }

    BFProgram { instructions: invec }
}

pub fn validate(instructions: String) {
    let mut nested : usize = 0;
    let mut row : usize = 0;
    let mut col : usize = 0;
    for c in instructions.as_bytes() {
        row += 1;
        match *c as char {
            LOOP_CH => nested += 1,
            ELOOP_CH => {
                if nested == 0 {
                    panic!("unmatched {ELOOP_CH} at {row}:{col}");
                }
                nested -= 1;
            },
            '\r' | '\n' => {
                col += 1;
                row = 0;
            },
            _ => continue,
        }
    }
}