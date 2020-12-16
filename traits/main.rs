struct User {
    first_name: String,
    last_name: String,
}

struct Car {
    name: String,
    color: String,
}

// Trait que tem método que retorna o nome completo, traits são similares a interfaces em outras linguagens
trait Fullname {
    fn get_fullname(&self) -> String;
}

// implementando trait Fullname ao tipo User
impl Fullname for User {
    fn get_fullname(&self) -> String {
        self.first_name.clone() + " " + &self.last_name
    }
}

// implementando trait Fullname ao tipo Car
impl Fullname for Car {
    fn get_fullname(&self) -> String {
        self.color.clone() + " " + &self.name
    }
}

// Essa função recebe um tipo genérico que tenha a trait Fullname implementada
fn show_fullname<T: Fullname>(obj: T) {
    println!("Fullname: {}", obj.get_fullname());
}

fn main() {
    let user = User {
        first_name: "Someone".to_string(),
        last_name: "Test".to_string(),
    };
    let car = Car {
        name: "Ferrari".to_string(),
        color: "Red".to_string(),
    };

    show_fullname(user);
    show_fullname(car);
}
