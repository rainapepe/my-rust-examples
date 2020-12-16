use std::ops::Add;

#[derive(Debug)]
struct Position {
    x: u32,
    y: u32,
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

fn sum<T: Add<Output = T>>(a: T, b: T) -> T {
    let result = a;
    result + b
}

fn main() {
    let a = Position { x: 10, y: 10 };
    let b = Position { x: 10, y: 10 };

    let result = sum(a, b);

    println!("result {:?}", result);
}
