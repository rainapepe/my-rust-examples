use std::mem::size_of;
use std::slice;

/**
   Example accessing value of a Struct Array from raw vector in memory
*/

struct Position {
    x: u8,
    y: u8,
}

impl Position {
    pub fn new(x: u8, y: u8) -> Position {
        Position { x, y }
    }
}

fn print_table(table: &[Position; 4]) {
    for (i, position) in table.iter().enumerate() {
        println!("Table[{}].x: {}", i, position.x);
        println!("Table[{}].y: {}", i, position.y);
    }
}

/**
   Get value of an element by accessing the position of the structure vector in memory.
   The array [Position;4] in memory is [u8;8], so accessing position 4 we will obtain the value "y"
   of the second element of the Position array ([Position;4])
*/
fn get_from_memory(table: &mut [Position; 4], addres: usize) -> u8 {
    let num_bytes = size_of::<[Position; 4]>();
    unsafe {
        let buffer = slice::from_raw_parts_mut(table as *mut [Position; 4] as *mut u8, num_bytes);
        buffer[addres]
    }
}

/**
* Same as get_from_memory, but setting a value at the position of the vector in memory
*/
fn set_from_memory(table: &mut [Position; 4], addres: usize, value: u8) {
    let num_bytes = size_of::<[Position; 4]>();
    unsafe {
        let buffer = slice::from_raw_parts_mut(table as *mut [Position; 4] as *mut u8, num_bytes);
        buffer[addres] = value;
    }
}

// creating a fake access of vector in memory
fn fake_get_from_memory(table: &mut [Position; 4], addres: usize) -> u8 {
    let prop = addres & 0x1; // the Position have 2 props, so we only use the final bit (0x1)
    let index = addres / 2; // get the position on [Position;4]
    match prop {
        // x prop
        0 => table[index].x,
        // y prop
        1 => table[index].y,
        // invalid
        _ => 0,
    }
}

fn main() {
    // this in memory = [u8;8] = [1,2,3,4,5,6,7,8];
    let mut table: [Position; 4] = [
        Position::new(1, 2),
        Position::new(3, 4),
        Position::new(5, 6),
        Position::new(7, 8),
    ];

    // print all elements
    print_table(&table);

    // unsafe - get direct from vector in memory
    println!("Pointer[5]: {}", get_from_memory(&mut table, 5));
    set_from_memory(&mut table, 5, 10);
    println!("Pointer[5]: {}", get_from_memory(&mut table, 5));

    // fake get direct from vector in memory
    println!("FakePointer[3]: {}", fake_get_from_memory(&mut table, 3));
    println!("FakePointer[6]: {}", fake_get_from_memory(&mut table, 6));

    // print all elements
    print_table(&table);
}
