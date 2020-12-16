pub fn say_your_name(name: &str) -> String {
    format!("My name is {}", name)
}

pub fn sum(n1: u32, n2: u32) -> u32 {
    n1 + n2
}

pub fn is_greater_than_100(value: u32) -> u32 {
    if value > 100 {
        panic!("Value is invalid");
    }

    value
}

fn main() {
    let n1 = 4;
    let n2 = 6;
    println!("{} + {} = {}", n1, n2, sum(n1, n2));
}

#[cfg(test)]
mod test;
