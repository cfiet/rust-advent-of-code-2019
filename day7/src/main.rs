extern crate intcode;
extern crate permutations;

use std::fs::read_to_string;

use intcode::Program;
use permutations::*;

const INPUT_PATH: &str = "day7/data/input.txt";

fn read_input() -> Result<Vec<i32>, std::io::Error> {
    let program = read_to_string(INPUT_PATH)?
        .split(',')
        .map(|v| v.parse::<i32>().expect("i32::parse"))
        .collect::<Vec<i32>>();

    Ok(program)
}

fn run_amplifiers(mem: &[i32], phase_settings: &[i32]) -> i32 {
    phase_settings.iter().fold(0i32, |acc, s| {
        let mut mem: Box<[i32]> = Box::from(mem);
        let p = Program::new(mem.as_mut())
            .add_input_value(*s)
            .add_input_value(acc)
            .run();
        let out = p.output().first().expect("No output produced");
        *out
    })
}

fn main() {
    let prog = read_input().unwrap();
    let max_out = (0..5)
        .collect::<Vec<i32>>()
        .unique_permutations()
        .map(|permutation| {
            let phase_settings = permutation.iter().cloned().copied().collect::<Vec<i32>>();

            run_amplifiers(&prog, &phase_settings)
        })
        .max()
        .expect("max()");

    println!("Max output: {}", max_out);
}
