use infra::{run_day, FromFile};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod infra;

fn main() {
    let day = Some(9);
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
        (FromFile("day2.txt"), 708),
    );

    run_day(
        day,
        1,
        day3::Day3,
        (FromFile("day3.txt"), 216),
        (FromFile("day3.txt"), 6708199680),
    );

    run_day(
        day,
        1,
        day4::Day4,
        (FromFile("day4.txt"), 250),
        (FromFile("day4.txt"), 158),
    );

    run_day(
        day,
        1,
        day5::Day5,
        (FromFile("day5.txt"), 832),
        (FromFile("day5.txt"), 517),
    );

    run_day(
        day,
        1,
        day6::Day6,
        (FromFile("day6.txt"), 6782),
        (FromFile("day6.txt"), 3596),
    );

    run_day(
        day,
        1,
        day7::Day7,
        (FromFile("day7.txt"), 128),
        (FromFile("day7.txt"), 20189),
    );

    run_day(
        day,
        1,
        day8::Day8,
        (FromFile("day8.txt"), 1814),
        (FromFile("day8.txt"), 1056),
    );

    run_day(
        day,
        1,
        day9::Day9,
        (FromFile("day9.txt"), 36845998),
        (FromFile("day9.txt"), 4830226),
    );
}
