use crate::compiler::{BFProgram, Instruction};

const MEMSIZE: usize = 30_000;

impl BFProgram {
    fn expose(&self) -> Vec<Instruction> {
        self.instructions.clone()
    }
}

pub fn interpret(prog: BFProgram) {
    use std::io::{Read, Write};
    use std::num::Wrapping;
    use Instruction::*;

    let mut ip: usize = 0;
    let mut dp: usize = 0;
    let mut d = [Wrapping(0u8); MEMSIZE];
    let mut stdin = std::io::stdin().bytes();
    let ins = prog.expose();
    let inslen = ins.len();

    while ip < inslen {
        match ins[ip] {
            LEFT => {
                if dp == 0 {
                    dp = MEMSIZE;
                } else {
                    dp -= 1;
                }
            }
            RIGHT => {
                dp += 1;
                if dp >= MEMSIZE {
                    dp = 0;
                }
            }
            INC => d[dp] += Wrapping(1u8),
            DEC => d[dp] -= Wrapping(1u8),
            IN => {
                d[dp] = Wrapping(
                    stdin
                        .next()
                        .expect("could not get input")
                        .ok()
                        .expect("could not get input"),
                )
            }
            OUT => {
                print!("{}", d[dp].0 as char);
                std::io::stdout().flush().expect("could not flush stdout");
            }
            LOOP(i) => {
                if d[dp].0 == 0 {
                    ip = i;
                }
            }
            ELOOP(i) => {
                if d[dp].0 != 0 {
                    ip = i;
                }
            }
            _ => continue,
        }

        ip += 1;
    }
}
