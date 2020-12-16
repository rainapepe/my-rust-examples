
// Nessa caso text1 e text2 foram movidas e serão destruidas no final da função e uma nova string é movida para quem chamou a função
fn concat_1(text1: String, text2: String) -> String {
    let mut result: String = text1.clone();
    result.push_str(&text2);
    result
}

// Nessa caso as apenas text1 foi movida para esse escopo e no final será movida de volta para quem chamou a função, text2 foi apenas emprestada
fn concat_2(text1: String, text2: &String) -> String {
    text1 + &text2
}

// Nesse caso text1 e text2 foram emprestadas, o valor da text1 foi copiado e adicionado concatenado com o text2 e no final será movida
fn concat_3(text1: &String, text2: &String) -> String {
    text1.clone() + " " + text2
}


fn main() {
    let first_name = "Raina".to_string();
    let last_name = "Pepe".to_string();

    let result1 = concat_1(first_name, last_name);
    println!("result 1 {}", result1);
    // As variaveis são invalidas, elas foram movidas para concat_1 e destruidas depois que a função terminou
    // println!("first_name {}", first_name);
    // println!("last_name {}", last_name);

    let first_name = "Raina".to_string();
    let last_name = "Pepe".to_string();

    let result2 = concat_2(first_name, &last_name);
    println!("result 2 {}", result2);
    // somente first_name é invalido pois foi movida e last_name foi apenas emprestada
    // println!("first_name {}", first_name);
    println!("last_name {}", last_name);

    let first_name = "Raina".to_string();
    let last_name = "Pepe".to_string();

    let result3 = concat_3(&first_name, &last_name);
    println!("result 3 {}", result3);

    // as duas variaveis foram emprestadas, então ainda estão validas nesse escopo
    println!("first_name {}", first_name);
    println!("last_name {}", last_name);
}
