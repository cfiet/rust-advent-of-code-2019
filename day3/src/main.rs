extern crate combine;

mod parser;

use std::{fs::File, io::BufReader};

use combine::{
    stream::{buffered, easy, position, ReadStream},
    Parser,
};

use parser::{wires, Wire};

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

fn main() {
    let input = read_input();
    println!("{:?}", input);
}
