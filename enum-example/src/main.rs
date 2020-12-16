enum Color {
    GREEN(u8, u8, u8),
    RED(String),
}

impl Color {
    fn get_hex(&self) -> String {
        match self {
            Color::GREEN(r, g, b) => format!("#{}{}{}", r, g, b),
            Color::RED(rgb) => rgb.clone(),
        }
    }
}

fn main() {
    let cor = Color::GREEN(0, 0, 0);
    let rgb = cor.get_hex();

    match rgb.as_str() {
        "#101010" => {
            println!("#101010");
        }
        "#000" => println!("is black"),
        _ => println!("is nothing"),
    }
}
