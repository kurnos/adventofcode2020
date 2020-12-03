use infra::{run_day, FromFile};

mod day1;
mod day2;
mod day3;
mod infra;

fn main() {
    let day = Some(3u8);
    run_day(
        day,
        1,
        day1::Day1,
        (FromFile("day1.txt"), 956091),
        (FromFile("day1.txt"), 79734368),
    );

    run_day(
        day,
        1,
        day2::Day2,
        (FromFile("day2.txt"), 519),
        (FromFile("day2.txt"), 708)
    );

    day3::main();
}
