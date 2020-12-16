// A convenient utility to convert variables into
// hex strings because "modern C++"'s method with
// streams is atrocious
// auto hex = [](uint32_t n, uint8_t d)
// {
//     std::string s(d, '0');
//     for (int i = d - 1; i >= 0; i--, n >>= 4)
//         s[i] = "0123456789ABCDEF"[n & 0xF];
//     return s;
// };
use std::ops::Range;
const HEX_TABLE: &str = "0123456789ABCDEF";

fn to_hex(n: u32, d: u8) -> String {
    let mut value = n;
    println!("d: {}", d);

    let mut text = Vec::<u8>::new(); // [0, d as usize];

    for i in (0..d).rev() {
        println!("i: {}", i);
        println!("value: {}", value);
        let c = HEX_TABLE.as_bytes()[(value & 0xF) as usize];

        text[i as usize] = c;
        value >>= 4;
    }

    String::from_utf8(text).unwrap()
}

fn main() {
    let result = to_hex(42, 4);

    println!("result: {}", result);
}
