use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let test_env = env::var("TEST").unwrap_or("Env not found".to_string());

    println!("Env TEST={}", test_env);

    let args = env::args();

    println!("args: ");
    for arg in args {
        println!("{}", arg);
    }
}
