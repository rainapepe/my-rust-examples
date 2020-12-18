const HEX_TABLE: &str = "0123456789ABCDEF";

fn to_hex(value: u32, hex_size: u8) -> String {
    let mut temp_value = value;
    let mut text = Vec::<u8>::new();

    // inicia o array com o tamanho do hex_size com zeros
    text.resize(hex_size as usize, 0);

    // começar do ultimo caracter para o primeiro
    for i in (0..hex_size).rev() {
        // identifica o caracter hex correpondente aos ultimos 4 bits
        let c = HEX_TABLE.as_bytes()[(temp_value & 0xF) as usize];
        text[i as usize] = c;

        // mover 4 bits para a direita para poder achar o valor do proximos 4 bits
        temp_value >>= 4;
    }

    String::from_utf8(text).unwrap()
}

fn main() {
    let a = to_hex(15, 4);
    let b = to_hex(255, 4);
    let c = to_hex(4369, 4);

    println!("15 => {}", a);
    println!("255 => {}", b);
    println!("4369 => {}", c);
}
