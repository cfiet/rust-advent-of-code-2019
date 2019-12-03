use combine::{
    error::ParseError,
    many1,
    parser::byte::{byte, crlf, digit, newline},
    sep_by, Parser, Stream,
};

pub type Section = (i32, i32);

fn section<Input>() -> impl Parser<Input, Output = Section>
where
    Input: Stream<Token = u8>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let up = byte(b'U').map(|_| (0, 1));
    let down = byte(b'D').map(|_| (0, -1));
    let left = byte(b'L').map(|_| (-1, 0));
    let right = byte(b'R').map(|_| (1, 0));

    let direction = up.or(down).or(left).or(right);

    let int =
        many1(digit()).map(|s: Vec<u8>| String::from_utf8(s).unwrap().parse::<i32>().unwrap());

    (direction, int).map(|((x, y), len)| (x * len, y * len))
}

pub type Wire = Vec<(i32, i32)>;

fn line<Input>() -> impl Parser<Input, Output = Wire>
where
    Input: Stream<Token = u8>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    sep_by(section(), byte(b','))
}

pub(crate) fn wires<Input>() -> impl Parser<Input, Output = (Wire, Wire)>
where
    Input: Stream<Token = u8>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (line(), newline().or(crlf()), line()).map(|(l1, _, l2)| (l1, l2))
}

#[cfg(test)]
mod test {
    use super::*;

    fn parses_section_correctly(input: &str, expected: (i32, i32)) {
        let stream = combine::stream::position::Stream::new(input.as_bytes());
        let parsed = section().parse(stream).unwrap().0;
        assert_eq!(parsed, expected);
    }

    #[test]
    fn check_section_parsing() {
        parses_section_correctly("U19", (0, 19));
        parses_section_correctly("D5", (0, -5));
        parses_section_correctly("L1", (-1, 0));
        parses_section_correctly("R4096", (4096, 0));
    }

    #[test]
    fn check_line_parsing() {
        let input = "U19,L1,D5,R4096";
        let stream = combine::stream::position::Stream::new(input.as_bytes());
        let parsed = line().parse(stream).unwrap().0;
        assert_eq!(parsed, vec![(0, 19), (-1, 0), (0, -5), (4096, 0)]);
    }

    #[test]
    fn check_wires_parsing() {
        let input = "U19,L1,D5,R4096\nR4096,D5,L1,U19";
        let stream = combine::stream::position::Stream::new(input.as_bytes());
        let parsed = wires().parse(stream).unwrap().0;
        assert_eq!(
            parsed,
            (
                vec![(0, 19), (-1, 0), (0, -5), (4096, 0)],
                vec![(4096, 0), (0, -5), (-1, 0), (0, 19)]
            )
        );
    }
}
