struct Ram {
    mem: [u8; 256],
}

impl<'a> Ram {
    pub fn new() -> Ram {
        Ram { mem: [0; 256] }
    }

    pub fn read(&mut self, addr: u8) -> u8 {
        self.mem[addr as usize]
    }

    pub fn write(&mut self, addr: u8, data: u8) {
        self.mem[addr as usize] = data;
    }
}
struct Cpu<'a> {
    ram: Option<&'a mut Ram>,
}

impl<'b> Cpu<'b> {
    pub fn new<'a>() -> Cpu<'a> {
        Cpu { ram: None }
    }
    pub fn connect_ram<'a: 'b>(&'a mut self, ram: &'a mut Ram) {
        self.ram = Some(ram);
    }

    pub fn read<'a>(&'a mut self, addr: u8) -> u8 {
        if let Some(ram) = &mut self.ram {
            return ram.read(addr);
        }

        return 0;
    }

    pub fn write<'a>(&'a mut self, addr: u8, data: u8) {
        if let Some(ram) = &mut self.ram {
            ram.write(addr, data);
        }
    }
}

fn read(ram: &mut Ram, addr: u8) -> u8 {
    return ram.read(addr);
}

fn write(ram: &mut Ram, addr: u8, data: u8) {
    ram.write(addr, data);
}

struct Bus<'a> {
    cpu: Cpu<'a>,
    ram: Ram,
}

impl<'a> Bus<'a> {
    pub fn new<'b: 'a>() -> Bus<'b> {
        let mut bus = Bus {
            cpu: Cpu::new(),
            ram: Ram::new(),
        };

        bus.cpu.connect_ram(&mut bus.ram);

        bus
    }
}

fn main() {
    let mut bus = Bus::new();

    // bus.cpu.connect_ram(&mut bus.ram);
    bus.ram.write(0x12, 0xFF);

    println!("Read from RAM: {}", bus.ram.read(0x12));
    println!("Read from CPU: {}", bus.cpu.read(0x12));

    // let mut ram = Ram::new();

    // write(&mut ram, 0x12, 215);
    // println!("Read: {}", read(&mut ram, 0x12));
}
