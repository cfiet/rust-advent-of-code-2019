extern crate intcode;

use std::clone::Clone;
use std::fs::read_to_string;

use intcode::Program;

const INPUT_PATH: &'static str = "day5/data/input.txt";

fn read_input() -> Vec<i32> {
    read_to_string(INPUT_PATH)
        .unwrap_or_else(|e| panic!("Error while reading file {}: {}", INPUT_PATH, e))
        .split(",")
        .map(|i| i.parse::<i32>().expect("parse<i32>"))
        .collect()
}

fn main() {
    let memory = read_input();

    let mut prog0 = memory.clone();
    let p = Program::new(&mut prog0).add_input(&[1i32]).run();
    println!("Output: {:?}", p.output());

    let mut prog1 = memory.clone();
    let p = Program::new(&mut prog1).add_input(&[5i32]).run();
    println!("Output: {:?}", p.output());
}

#[cfg(test)]
mod test {
    use intcode::Program;

    #[test]
    fn test_position_mode() {
        vec![(0, 0), (215, 1)].iter().for_each(|(input, output)| {
            let mem = &mut [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
            let p = Program::new(mem).add_input(&[*input]).run();
            assert_eq!(p.output(), &[*output]);
        });
    }

    #[test]
    fn test_immediate_mode() {
        vec![(0, 0), (215, 1)].iter().for_each(|(input, output)| {
            let mem = &mut [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
            let p = Program::new(mem).add_input(&[*input]).run();
            assert_eq!(p.output(), &[*output]);
        });
    }

    #[test]
    fn test_complex_example() {
        vec![(7, 999), (8, 1000), (9, 1001)]
            .iter()
            .for_each(|(input, output)| {
                let mem = &mut [
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
                ];
                let p = Program::new(mem).add_input(&[*input]).run();
                assert_eq!(p.output(), &[*output]);
            });
    }
}
