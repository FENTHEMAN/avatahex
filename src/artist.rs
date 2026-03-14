pub const WIDTH: isize = 400;
pub const HEIGHT: isize = WIDTH;

pub const HOME_Y: isize = HEIGHT / 2;
pub const HOME_X: isize = WIDTH / 2;

pub const STROKE_WIDTH: usize = 5;

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Forward(isize),
    TurnLeft,
    TurnRight,
    Home,
    Noop(u8),
}

#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    West,
    East,
    South,
    North,
}

#[derive(Debug)]
pub struct Artist {
    pub x: isize,
    pub y: isize,
    pub heading: Orientation,
}

impl Artist {
    pub fn new() -> Self {
        Artist {
            x: HOME_X,
            y: HOME_Y,
            heading: Orientation::North,
        }
    }

    pub fn home(&mut self) {
        self.x = HOME_X;
        self.y = HOME_Y;
    }

    pub fn forward(&mut self, distance: isize) {
        match self.heading {
            Orientation::North => self.y += distance,
            Orientation::South => self.y -= distance,
            Orientation::West => self.x += distance,
            Orientation::East => self.x -= distance,
        }
    }

    pub fn turn_right(&mut self) {
        self.heading = match self.heading {
            Orientation::North => Orientation::East,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
            Orientation::East => Orientation::South,
        }
    }

    pub fn turn_left(&mut self) {
        self.heading = match self.heading {
            Orientation::North => Orientation::West,
            Orientation::South => Orientation::East,
            Orientation::West => Orientation::South,
            Orientation::East => Orientation::North,
        }
    }

    pub fn wrap(&mut self) {
        if self.x < 0 {
            self.x = HOME_X;
            self.heading = Orientation::West;
        } else if self.x > WIDTH {
            self.x = HOME_X;
            self.heading = Orientation::East;
        }

        if self.y < 0 {
            self.y = HOME_Y;
            self.heading = Orientation::North;
        } else if self.y > HEIGHT {
            self.y = HOME_Y;
            self.heading = Orientation::South;
        }
    }
}
