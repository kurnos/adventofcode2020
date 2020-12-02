use infra::{run_day, FromFile};

mod day1;
mod infra;

fn main() {
    run_day(
        None,
        1,
        day1::Day1,
        (FromFile("day1.txt"), 956091),
        (FromFile("day1.txt"), 79734368),
    );
}
