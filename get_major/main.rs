fn get_major<'a>(a: &'a String, b: &'a String) -> Option<&'a String> {
    match a.len() as i32 - b.len() as i32 {
        x if x < 0 => Some(b),
        0 => None,
        _ => Some(a),
    }
}

fn main() {
    let a = "teste maior".to_string();
    let b = "teste".to_string();

    let result = get_major(&a, &b);

    println!(
        "a: {}, b: {}, maior: {}",
        a,
        b,
        result.unwrap_or(&"strings iguais".to_string())
    );
}
