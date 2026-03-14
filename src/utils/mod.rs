mod byte;
mod queue_optimisation;

use std::iter::once;

use crate::{artist::{Artist, HEIGHT, HOME_X, HOME_Y, Operation, STROKE_WIDTH, WIDTH}, utils::byte::parse_byte};
use rayon::prelude::*;
use svg::{
    Document,
    node::element::{
        Path, Rectangle,
        path::{Command, Data, Position},
    },
};

pub fn rayon_parse(input: &str) -> Vec<Operation> {
    input
        .as_bytes()
        .par_iter()
        .map(|byte| parse_byte(*byte))
        .collect()
}

pub fn parse(input: &str) -> Vec<Operation> {
    input
        .bytes()
        .map(parse_byte)
        .collect()
}

pub use queue_optimisation::queue_parse;

pub fn convert(operations: &Vec<Operation>) -> Vec<Command> {
    let mut artist = Artist::new();

    once(Command::Move(Position::Absolute, (HOME_X, HOME_Y).into()))
        .chain(operations.iter().map(|operation| {
            match *operation {
                Operation::Forward(distance) => artist.forward(distance),
                Operation::Noop(byte) => eprintln!("Failed to read byte from input: {byte}"),
                Operation::Home => artist.home(),
                Operation::TurnLeft => artist.turn_left(),
                Operation::TurnRight => artist.turn_right(),
            }
            let command = Command::Line(Position::Absolute, (artist.x, artist.y).into());
            artist.wrap();
            command
        }))
        .collect()
}

pub fn generate_svg(commands: Vec<Command>) -> Document {
    let background = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", WIDTH)
        .set("height", HEIGHT)
        .set("fill", "#ffffff");

    let border = background
        .clone()
        .set("fill-opacity", "0.0")
        .set("stroke", "#cccccc")
        .set("stroke-width", 3 * STROKE_WIDTH);

    let sketch = Path::new()
        .set("fill", "none")
        .set("stroke", "#2f2f2f")
        .set("stroke-width", STROKE_WIDTH)
        .set("stroke-opacity", "0.9")
        .set("d", Data::from(commands));

    let document = Document::new()
        .set("viewBox", (0, 0, HEIGHT, WIDTH))
        .set("height", HEIGHT)
        .set("width", WIDTH)
        .set("style", "style=\"outline: 5px solid #800000;\"")
        .add(background)
        .add(sketch)
        .add(border);

    document
}
