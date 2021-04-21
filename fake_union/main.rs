struct Status {
    register: u8,
    // a: 4 bits, bits acima são os menos significantes
    // b: 2 bit,
    // c: 2 bit,
}

fn get_bits_u8(value: u8, position: u8, bits: u8) -> u8 {
    (value >> position) & (255 >> (8 - bits))
}

fn set_bits_u8(value: u8, position: u8, bits: u8, data: u8) -> u8 {
    let sub = get_bits_u8(value, position, bits) << position;
    let add = (data & (255 >> (8 - bits))) << position;

    (value - sub) + add
}

impl Status {
    pub fn new() -> Status {
        Status { register: 0 }
    }

    pub fn get_a(&self) -> u8 {
        get_bits_u8(self.register, 0, 4)
    }

    pub fn set_a(&mut self, data: u8) {
        self.register = set_bits_u8(self.register, 0, 4, data);
    }

    pub fn get_b(&self) -> u8 {
        get_bits_u8(self.register, 4, 2)
    }

    pub fn set_b(&mut self, data: u8) {
        self.register = set_bits_u8(self.register, 4, 2, data);
    }

    pub fn get_c(&self) -> u8 {
        get_bits_u8(self.register, 6, 2)
    }

    pub fn set_c(&mut self, data: u8) {
        self.register = set_bits_u8(self.register, 6, 2, data);
    }

    pub fn print(&self) {
        println!("register: {}", self.register);
        println!("a: {}", self.get_a());
        println!("b: {}", self.get_b());
        println!("c: {}", self.get_c());
    }
}

fn main() {
    let mut status = Status::new();

    status.register = 0b11101010;
    // status.register = 255;

    status.print();
    status.set_a(15);
    status.print();
    status.set_b(3);
    status.print();
    status.set_c(1);
    status.print();
}
