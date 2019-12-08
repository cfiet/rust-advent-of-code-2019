use std::borrow::Borrow;
use std::convert::TryFrom;
use std::fmt::Debug;

pub struct Program<'a> {
    memory: &'a mut [i32],
    next_op: usize,
}

impl<'a> Program<'a> {
    pub fn new<'b>(memory: &'b mut [i32]) -> Program<'b> {
        Program { memory, next_op: 0 }
    }

    fn read_at(&self, pos: usize) -> i32 {
        *self.memory.get(pos).expect(&format!("read_at({})", pos))
    }

    fn parse_op(&self) -> Result<Op, OpError> {
        use OpError::*;

        let opcode_val = self.read_at(self.next_op);
        let opcode = OpCode::try_from(opcode_val).map_err(InvalidOpCode)?;

        match opcode.op {
            99 => Ok(Op::Terminate),
            1 => {
                let left = self.parse_param(1, opcode.arg0)?;
                let right = self.parse_param(2, opcode.arg1)?;
                match self.parse_param(3, opcode.arg2)? {
                    Param::Immediate(_) => Err(ImmediateTargetParam(3)),
                    Param::Position(target) => Ok(Op::Add((left, right, target))),
                }
            }
            2 => {
                let left = self.parse_param(1, opcode.arg0)?;
                let right = self.parse_param(2, opcode.arg1)?;
                match self.parse_param(3, opcode.arg2)? {
                    Param::Immediate(_) => Err(ImmediateTargetParam(3)),
                    Param::Position(target) => Ok(Op::Mul((left, right, target))),
                }
            }
            _ => Err(UnrecognisedOpCode(opcode.op)),
        }
    }

    fn parse_param(&self, offset: usize, mode: ParamMode) -> Result<Param, OpError> {
        let address = self.next_op + offset;
        let value = self.read_at(address);

        match mode {
            ParamMode::Immediate => Ok(Param::Immediate(value)),
            ParamMode::Position => {
                if value < 0 || value >= self.memory.len() as i32 {
                    Err(OpError::PositionParamOutOfBounds(offset, value))
                } else {
                    Ok(Param::Position(value as usize))
                }
            }
        }
    }

    fn read_param(&self, param: &Param) -> i32 {
        match param {
            Param::Immediate(value) => *value,
            Param::Position(pos) => self.read_at(*pos),
        }
    }

    fn step(&mut self) -> Option<usize> {
        let op = self
            .parse_op()
            .unwrap_or_else(|e| panic!("Error while running op at {}: {:?}", self.next_op, e));

        match &op {
            Op::Add((p0, p1, dest)) => {
                let left = self.read_param(p0);
                let right = self.read_param(p1);
                self.memory[*dest] = left + right;
            }
            Op::Mul((p0, p1, dest)) => {
                let left = self.read_param(p0);
                let right = self.read_param(p1);
                self.memory[*dest] = left * right;
            }
            _ => {}
        };

        let jmp = op.size();
        self.next_op += jmp.unwrap_or(0);

        jmp
    }

    pub fn run(mut self) -> Program<'a> {
        'main: loop {
            if self.step() == None {
                break 'main;
            }
        }

        self
    }
}

#[derive(Debug)]
enum OpError {
    UnrecognisedOpCode(u8),
    PositionParamOutOfBounds(usize, i32),
    ImmediateTargetParam(usize),
    InvalidOpCode(OpCodeError),
}

#[derive(Debug)]
enum Op {
    Add((Param, Param, usize)),
    Mul((Param, Param, usize)),
    Input(usize),
    Output(usize),
    Terminate,
}

impl Op {
    fn size(&self) -> Option<usize> {
        match self {
            Op::Add(_) | Op::Mul(_) => Some(4),
            Op::Input(_) | Op::Output(_) => Some(2),
            Op::Terminate => None,
        }
    }
}

#[derive(Debug)]
enum Param {
    Position(usize),
    Immediate(i32),
}

#[derive(Debug, PartialEq)]
enum ParamModeError {
    UnrecognisedMode(i32),
}

#[derive(Debug, PartialEq)]
enum ParamMode {
    Position,
    Immediate,
}

impl ParamMode {
    fn as_int(&self) -> i32 {
        match self {
            ParamMode::Position => 0,
            ParamMode::Immediate => 1,
        }
    }
}

impl TryFrom<i32> for ParamMode {
    type Error = ParamModeError;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(ParamMode::Position),
            1 => Ok(ParamMode::Immediate),
            n => Err(ParamModeError::UnrecognisedMode(n)),
        }
    }
}

#[derive(Debug, PartialEq)]
struct OpCode {
    op: u8,
    arg0: ParamMode,
    arg1: ParamMode,
    arg2: ParamMode,
}

const ARG2_MASK: i32 = 10_000;
const ARG1_MASK: i32 = 1_000;
const ARG0_MASK: i32 = 100;

#[derive(Debug, PartialEq)]
enum OpCodeError {
    InvalidOpcode(i32),
    InvalidParamMode(usize, ParamModeError),
}

impl TryFrom<i32> for OpCode {
    type Error = OpCodeError;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        use OpCodeError::*;

        let mut value: i32 = *v.borrow();

        let arg2 = ParamMode::try_from(value / ARG2_MASK).map_err(|e| InvalidParamMode(2, e))?;
        value -= ARG2_MASK * arg2.as_int();

        let arg1 = ParamMode::try_from(value / ARG1_MASK).map_err(|e| InvalidParamMode(1, e))?;
        value -= ARG1_MASK * arg1.as_int();

        let arg0 = ParamMode::try_from(value / ARG0_MASK).map_err(|e| InvalidParamMode(0, e))?;
        value -= ARG0_MASK * arg0.as_int();

        if value < 100 && value > 0 {
            Ok(OpCode {
                op: value as u8,
                arg0,
                arg1,
                arg2,
            })
        } else {
            Err(InvalidOpcode(value))
        }
    }
}

#[cfg(test)]
mod tests {

    use super::ParamMode::{Immediate as I, Position as P};
    use super::*;

    #[test]
    fn opcode_parsing_works_for_all_combinations_of_arguments() {
        vec![
            (
                10010,
                OpCode {
                    op: 10,
                    arg0: P,
                    arg1: P,
                    arg2: I,
                },
            ),
            (
                1009,
                OpCode {
                    op: 9,
                    arg0: P,
                    arg1: I,
                    arg2: P,
                },
            ),
            (
                188,
                OpCode {
                    op: 88,
                    arg0: I,
                    arg1: P,
                    arg2: P,
                },
            ),
            (
                11111,
                OpCode {
                    op: 11,
                    arg0: I,
                    arg1: I,
                    arg2: I,
                },
            ),
        ]
        .iter()
        .for_each(|(val, exp)| {
            let result = OpCode::try_from(*val);
            assert_eq!(&result.unwrap(), exp, "Error parsing opcode {}", val);
        });
    }

    #[test]
    fn check_simple_add_program() {
        let mut mem = vec![1101, 2, 3, 3, 99];
        Program::new(&mut mem).run();
        assert_eq!(
            *mem.get(3).unwrap(),
            5,
            "Invalid sum value in memory position 3"
        );
    }

    #[test]
    fn check_simple_mul_program() {
        let mut mem = vec![1102, 2, 3, 3, 99];
        Program::new(&mut mem).run();
        assert_eq!(
            *mem.get(3).unwrap(),
            6,
            "Invalid mul value in memory position 3"
        );
    }
}
