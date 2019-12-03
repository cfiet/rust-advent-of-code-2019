use std::borrow::BorrowMut;
use std::clone::Clone;
use std::fs::read_to_string;
use std::ops::{Deref, DerefMut};

static INPUT_PATH: &str = "day2/data/input.txt";

struct Program<Mem: BorrowMut<Vec<usize>>> {
    memory: Mem,
    next_instruction: usize,
}

impl<Mem: BorrowMut<Vec<usize>>> Program<Mem> {
    fn new(memory: Mem) -> Program<Mem> {
        Program {
            memory,
            next_instruction: 0,
        }
    }

    fn memory(self) -> Mem {
        self.memory
    }

    fn run(mut self) -> Program<Mem> {
        loop {
            if self.step() == None {
                break;
            }
        }

        self
    }

    fn read_memory(&self, pos: &usize) -> &usize {
        self.memory.borrow().deref().get(*pos).unwrap()
    }

    fn step(&mut self) -> Option<usize> {
        let instruction = self.read_memory(&self.next_instruction).clone();
        match instruction {
            99 => None,
            1 | 2 => {
                let (result, target) = {
                    let op = self.get_operands();
                    let a0 = self.read_memory(op.0);
                    let a1 = self.read_memory(op.1);
                    (if instruction == 1 { a0 + a1 } else { a0 * a1 }, *op.2)
                };
                self.memory.borrow_mut().deref_mut()[target] = result;
                self.next_instruction += 4;
                Some(instruction as usize)
            }
            _ => panic!("Unknown operant: {}", instruction),
        }
    }

    fn get_operands(&self) -> (&usize, &usize, &usize) {
        (
            self.read_memory(&(self.next_instruction + 1)),
            self.read_memory(&(self.next_instruction + 2)),
            self.read_memory(&(self.next_instruction + 3)),
        )
    }
}

fn read_input() -> Vec<usize> {
    read_to_string(INPUT_PATH)
        .unwrap_or_else(|e| panic!("Error reading file {}: {}", INPUT_PATH, e))
        .split(",")
        .map(|s| {
            s.parse::<usize>()
                .unwrap_or_else(|e| panic!("Unable to convert {} to u32: {}", s, e))
        })
        .collect()
}

fn find_noun_verb(program: Vec<usize>, expected: usize) -> Option<(usize, usize)> {
    for i in 0..99 {
        for j in 0..99 {
            let mut p2_in = program.clone();
            p2_in[1] = i;
            p2_in[2] = j;

            if Program::new(p2_in).run().memory().get(0).unwrap().clone() == expected {
                return Some((i, j));
            }
        }
    }

    None
}

fn main() {
    let input = read_input();
    let mut p1_in = input.clone();
    p1_in[1] = 12;
    p1_in[2] = 2;
    let program = Program::new(p1_in).run();
    println!("Program output is: {}", program.memory().get(0).unwrap());

    let pair = find_noun_verb(input, 19690720).unwrap();
    println!("Noun and verb result: {}", 100 * pair.0 + pair.1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_day2_examples() {
        vec![
            (vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]),
            (vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]),
            (vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]),
            (
                vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
                vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            ),
        ]
        .iter_mut()
        .for_each(|(input, out)| assert_eq!(Program::new(input).run().memory(), out))
    }
}
