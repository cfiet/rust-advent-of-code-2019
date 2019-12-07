extern crate combine;

mod parser;

use std::{clone::Clone, fmt::Debug, fs::File, io::BufReader};

use combine::{
    stream::{buffered, easy, position, ReadStream},
    Parser,
};

use parser::{wires, Wire};

use self::SectionDirection::*;

static INPUT_PATH: &str = "day3/data/input.txt";

fn read_input() -> (Wire, Wire) {
    let file = File::open(INPUT_PATH)
        .unwrap_or_else(|err| panic!("Error while opening {}: {}", INPUT_PATH, err));
    let reader = BufReader::new(file);
    let stream = easy::Stream::from(buffered::Stream::new(
        position::Stream::new(ReadStream::new(reader)),
        1,
    ));

    wires()
        .parse(stream)
        .unwrap_or_else(|err| panic!("Error while parsing {}: {:?}", INPUT_PATH, err))
        .0
}

#[derive(Debug)]
enum SectionDirection {
    Horizontal,
    Vertical,
    Diagonal,
}

trait Section {
    fn start_x(&self) -> i32;
    fn start_y(&self) -> i32;

    fn end_x(&self) -> i32;
    fn end_y(&self) -> i32;

    fn direction(&self) -> SectionDirection;
}

impl Section for ((i32, i32), (i32, i32)) {
    fn start_x(&self) -> i32 {
        let start = self.0;
        start.0
    }

    fn start_y(&self) -> i32 {
        let start = self.0;
        start.1
    }

    fn end_x(&self) -> i32 {
        let end = self.1;
        end.0
    }

    fn end_y(&self) -> i32 {
        let end = self.1;
        end.1
    }

    fn direction(&self) -> SectionDirection {
        if self.start_y() == self.end_y() {
            SectionDirection::Horizontal
        } else if self.start_x() == self.end_x() {
            SectionDirection::Vertical
        } else {
            SectionDirection::Diagonal
        }
    }
}

fn into_absolute_coords(w: &Wire) -> Vec<((i32, i32), (i32, i32))> {
    w.iter()
        .scan((0, 0), |state, p| {
            let dest = (state.0 + p.0, state.1 + p.1);
            let res = if state.1 == dest.1 {
                if state.0 < dest.0 {
                    (*state, dest)
                } else {
                    (dest, *state)
                }
            } else if state.0 == dest.0 {
                if state.1 < dest.1 {
                    (*state, dest)
                } else {
                    (dest, *state)
                }
            } else {
                panic!("Unexpected section points: {:?}, {:?}", state, dest);
            };

            *state = dest;
            Some(res)
        })
        .collect()
}

fn find_closest_intesection<T: Section + Debug>(wire1: &Vec<T>, wire2: &Vec<T>) -> i32 {
    let mut current_closest_distance = std::i32::MAX;
    let mut current_closest_point = (0, 0);

    wire1
        .iter()
        .flat_map(|s1| {
            wire2
                .iter()
                .filter_map(move |s2| match (s1.direction(), s2.direction()) {
                    (Horizontal, Vertical) => Some((s1, s2)),
                    (Vertical, Horizontal) => Some((s2, s1)),
                    _ => None,
                })
        })
        .filter_map(|(s1, s2)| {
            if s1.start_x() <= s2.start_x()
                && s1.end_x() >= s2.start_x()
                && s2.start_y() <= s1.start_y()
                && s2.end_y() >= s1.start_y()
            {
                Some((s2.start_x(), s1.start_y()))
            } else {
                None
            }
        })
        .for_each(|intersection| {
            let distance = intersection.0.abs() + intersection.1.abs();
            if distance < current_closest_distance {
                current_closest_distance = distance;
                current_closest_point = intersection;
            }
        });

    current_closest_distance
}

fn main() {
    let input = read_input();

    let line0 = into_absolute_coords(&input.0);
    let line1 = into_absolute_coords(&input.1);

    let dist = find_closest_intesection(&line0, &line1);
    println!("Closest distance: {}", dist)
}
