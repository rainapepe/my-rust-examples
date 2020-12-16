mod schedule;
mod user;

use schedule::schedules;
use user::{model_user, User};

fn main() {
    let new_user = model_user::create_user("raina".to_string(), 23);
    let new_schedule = schedules::create("backup".to_string());

    println!("User: {}", new_user.name);
    println!("Schedule: {}", new_schedule.name);
}
