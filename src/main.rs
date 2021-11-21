use chrono::prelude::*;

fn main() {
    let end = Local.ymd(2022, 11, 17);
    let today = Local::today();
    let duration = end - today;
    let left_of_days = duration.num_days();

    println!("{}", left_of_days);
}
