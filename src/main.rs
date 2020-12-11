use infra::{run_day, FromFile};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod infra;

fn main() {
    let day = Some(11);
    run_day(
        day,
        1,
        day01::Day1,
        (FromFile("day1.txt"), 956091),
        (FromFile("day1.txt"), 79734368),
    );

    run_day(
        day,
        1,
        day02::Day2,
        (FromFile("day2.txt"), 519),
        (FromFile("day2.txt"), 708),
    );

    run_day(
        day,
        1,
        day03::Day3,
        (FromFile("day3.txt"), 216),
        (FromFile("day3.txt"), 6708199680),
    );

    run_day(
        day,
        1,
        day04::Day4,
        (FromFile("day4.txt"), 250),
        (FromFile("day4.txt"), 158),
    );

    run_day(
        day,
        1,
        day05::Day5,
        (FromFile("day5.txt"), 832),
        (FromFile("day5.txt"), 517),
    );

    run_day(
        day,
        1,
        day06::Day6,
        (FromFile("day6.txt"), 6782),
        (FromFile("day6.txt"), 3596),
    );

    run_day(
        day,
        1,
        day07::Day7,
        (FromFile("day7.txt"), 128),
        (FromFile("day7.txt"), 20189),
    );

    run_day(
        day,
        1,
        day08::Day8,
        (FromFile("day8.txt"), 1814),
        (FromFile("day8.txt"), 1056),
    );

    run_day(
        day,
        1,
        day09::Day9,
        (FromFile("day9.txt"), 36845998),
        (FromFile("day9.txt"), 4830226),
    );

    run_day(
        day,
        1,
        day10::Day10,
        (FromFile("day10.txt"), 2263),
        (FromFile("day10.txt"), 396857386627072),
    );

    run_day(
        day,
        1,
        day11::Day11,
        (FromFile("day11.txt"), 2361),
        (FromFile("day11.txt"), 2119),
    );
}
