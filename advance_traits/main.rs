trait Collection {
    fn get_collection() -> &'static str;
}

trait Model
where
    Self: Collection,
{
    const Collection: &'static str;

    type Schema;

    fn find() -> Self::Schema;
}

struct User {
    name: &'static str,
}

impl Collection for User {
    fn get_collection() -> &'static str {
        "user"
    }
}

impl Model for User {
    const Collection: &'static str = "user";
    type Schema = String;

    fn find() -> Self::Schema {
        "teste".to_string()
    }
}

fn main() {}
