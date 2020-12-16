use std::ops::Add;

struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn new(x: u32, y: u32) -> Position {
        Position { x: x, y: y }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Position::new(10, 10);
    let p2 = Position::new(20, 20);

    let result = p1 + p2;

    println!("result ({},{})", result.x, result.y);
}
