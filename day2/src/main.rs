use std::clone::Clone;
use std::fs::read_to_string;

use intcode::Program;

static INPUT_PATH: &str = "day2/data/input.txt";

fn read_input() -> Vec<i32> {
    read_to_string(INPUT_PATH)
        .unwrap_or_else(|e| panic!("Error reading file {}: {}", INPUT_PATH, e))
        .split(",")
        .map(|s| {
            s.parse::<i32>()
                .unwrap_or_else(|e| panic!("Unable to convert {} to u32: {}", s, e))
        })
        .collect()
}

fn find_noun_verb(program: &Vec<i32>, expected: i32) -> Option<(i32, i32)> {
    for i in 0..99 {
        for j in 0..99 {
            let mut p2_in = program.clone();
            p2_in[1] = i;
            p2_in[2] = j;

            Program::new(&mut p2_in).run();

            if *p2_in.get(0).expect("get(0)") == expected {
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

    Program::new(&mut p1_in).run();
    println!("Program output is: {}", p1_in.get(0).expect("get(0)"));

    let pair = find_noun_verb(&input, 19_690_720).unwrap();
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
        .for_each(|(input, out): &mut (Vec<i32>, Vec<i32>)| {
            Program::new(input).run();
            assert_eq!(input, out);
        });
    }
}
