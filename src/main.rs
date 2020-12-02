use infra::{run_day, FromFile};

mod day1;
mod day2;
mod infra;

fn main() {
    run_day(
        None,
        1,
        day1::Day1,
        (FromFile("day1.txt"), 956091),
        (FromFile("day1.txt"), 79734368),
    );

    run_day(
        None,
        1,
        day2::Day2,
        (FromFile("day2.txt"), 519),
        (FromFile("day2.txt"), 708)
    );
}
