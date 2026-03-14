use crate::artist::{HEIGHT, Operation};

pub fn parse_byte(byte: u8) -> Operation {
    match byte {
        b'0' => Operation::Home,
        b'1'..=b'9' => {
            let distance = (byte - 0x30) as isize;
            Operation::Forward(distance * (HEIGHT / 10))
        }
        b'a' | b'b' | b'c' => Operation::TurnLeft,
        b'd' | b'e' | b'f' => Operation::TurnRight,
        _ => Operation::Noop(byte),
    }
}
