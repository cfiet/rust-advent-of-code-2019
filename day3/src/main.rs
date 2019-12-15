extern crate combine;

mod parser;

use std::{clone::Clone, convert::From, fmt::Debug, fs::File, io::BufReader, marker::Copy};

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

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Position {
    fn from(v: (i32, i32)) -> Self {
        Position { x: v.0, y: v.1 }
    }
}

#[derive(Debug)]
struct Section {
    start: Position,
    end: Position,
    distance: i32,
}

impl Section {
    fn new(start: Position, end: Position, distance: i32) -> Section {
        Section {
            start,
            end,
            distance,
        }
    }

    fn direction(&self) -> SectionDirection {
        if self.start.y == self.end.y {
            SectionDirection::Horizontal
        } else if self.start.x == self.end.x {
            SectionDirection::Vertical
        } else {
            SectionDirection::Diagonal
        }
    }
}

fn into_absolute_coords(w: &Wire) -> Vec<Section> {
    #[derive(Debug)]
    struct State {
        pos: Position,
        dist: i32,
    }

    impl State {
        fn empty() -> State {
            State {
                pos: Position::from((0, 0)),
                dist: 0,
            }
        }
    }

    w.iter()
        .scan(State::empty(), |state, p| {
            let dest = Position::from((state.pos.x + p.0, state.pos.y + p.1));
            let res = Section::new(state.pos, Position::from(dest), state.dist);
            state.pos = dest;
            state.dist += p.0.abs() + p.1.abs();
            Some(res)
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct Intersection {
    pos: Position,
    dist: i32,
}

impl Intersection {
    fn empty() -> Intersection {
        Intersection {
            pos: Position::from((0, 0)),
            dist: std::i32::MAX,
        }
    }
}

fn find_intesections(wire1: &Vec<Section>, wire2: &Vec<Section>) -> Vec<Intersection> {
    wire1
        .iter()
        .skip(1)
        .flat_map(|s1| {
            wire2
                .iter()
                .skip(1)
                .filter_map(move |s2| match (s1.direction(), s2.direction()) {
                    (Horizontal, Vertical) => Some((s1, s2)),
                    (Vertical, Horizontal) => Some((s2, s1)),
                    _ => None,
                })
        })
        .filter_map(|(s1, s2)| {
            let (left_x, right_x) = if s1.start.x <= s1.end.x {
                (s1.start.x, s1.end.x)
            } else {
                (s1.end.x, s1.start.x)
            };
            let (top_y, bot_y) = if s2.start.y <= s2.end.y {
                (s2.start.y, s2.end.y)
            } else {
                (s2.end.y, s2.start.y)
            };

            if left_x <= s2.start.x
                && right_x >= s2.start.x
                && top_y <= s1.start.y
                && bot_y >= s1.start.y
            {
                Some(Intersection {
                    pos: Position::from((s2.start.x, s1.start.y)),
                    dist: s1.distance
                        + (s2.start.x - s1.start.x).abs()
                        + s2.distance
                        + (s1.start.y - s2.start.y).abs(),
                })
            } else {
                None
            }
        })
        .collect()
}

fn find_closest_start(intesections: &[Intersection]) -> Intersection {
    intesections.iter().fold(Intersection::empty(), |acc, i| {
        let dist = i.pos.x.abs() + i.pos.y.abs();
        if dist < acc.dist {
            Intersection { pos: i.pos, dist }
        } else {
            acc
        }
    })
}

fn find_shortest_start(intersections: &[Intersection]) -> Intersection {
    intersections.iter().fold(Intersection::empty(), |acc, i| {
        if i.dist < acc.dist {
            *i
        } else {
            acc
        }
    })
}

fn main() {
    let input = read_input();

    let line0 = into_absolute_coords(&input.0);
    let line1 = into_absolute_coords(&input.1);

    let intersections = find_intesections(&line0, &line1);
    let closest_start = find_closest_start(&intersections);
    println!("Closest distance: {}", closest_start.dist);

    let shortest_dist = find_shortest_start(&intersections);
    println!("Shortest distance: {}", shortest_dist.dist);
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_intersections(input: &str) -> Vec<Intersection> {
        let res = wires()
            .parse(combine::stream::position::Stream::new(input.as_bytes()))
            .unwrap()
            .0;

        let line0 = into_absolute_coords(&res.0);
        let line1 = into_absolute_coords(&res.1);

        find_intesections(&line0, &line1)
    }

    fn run_part1_example(input: &str, n: i32) {
        let intersections = get_intersections(input);
        let closest_start = find_closest_start(&intersections);
        assert_eq!(closest_start.dist, n);
    }

    fn run_part2_example(input: &str, n: i32) {
        let intersections = get_intersections(input);
        let shortest_start = find_shortest_start(&intersections);
        assert_eq!(shortest_start.dist, n);
    }

    #[test]
    fn test_closest_examples() {
        vec![
            (
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83",
                159,
            ),
            (
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                135,
            ),
        ]
        .iter()
        .for_each(|(input, expected)| run_part1_example(input, *expected));
    }

    #[test]
    fn test_shortest_examples() {
        vec![
            ("U3,L5\nL2,U1,R1,U1,L1,U3", 12),
            ("U10,L15\nL10,U15", 40),
            ("D10,L15\nL10,D15", 40),
            ("U10,R15\nR10,U15", 40),
            ("D10,R15\nR10,D15", 40),
            (
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83",
                610,
            ),
            (
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                410,
            ),
        ]
        .iter()
        .for_each(|(input, expected)| run_part2_example(input, *expected));
    }
}
