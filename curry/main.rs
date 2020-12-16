
fn sum_with(a: u32) -> impl Fn(u32) -> u32 {
    // move é para mover o valor de a para o escopo do closure
    move |b| a + b
}

fn main() {
    let sum_with_4 = sum_with(4);

    println!("Teste: {}", sum_with_4(6));
    println!("Teste: {}", sum_with_4(15));
}