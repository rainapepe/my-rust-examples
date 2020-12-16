// Essa função recebe uma lista de um tipo generico que só aceita os tipos que tem as traits Copy e PartialOrd implementadas
// Copy: trait que implementa o método de copiar, tipos com essa trait podem ser copiados como um numbers e char
// PartialOrd: trait que implementa o métodos para ordenação como > e <
fn major_in_list<T: Copy + PartialOrd>(list: &[T]) -> T {
    let mut major = list[0];

    for &item in list.iter() {
        if item > major {
            major = item;
        }
    }

    major
}

// Outra forma de definir limites de traits a um tipo genérico é utilizando o where
fn major_in_list_2<T>(list: &[T]) -> T
where
    T: Copy + PartialOrd,
{
    let mut major = list[0];

    for &item in list.iter() {
        if item > major {
            major = item;
        }
    }

    major
}

fn main() {
    let list_u32 = vec![1, 3, 2, 7, 5, 4];

    let major_u32 = major_in_list(&list_u32);

    println!("Major u32 {}", major_u32);

    let list_char = vec!['y', 'm', 'w', 'a', 'q'];

    let major_char = major_in_list_2(&list_char);
    println!("Major char {}", major_char);
}
