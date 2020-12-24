#[allow(unused_imports)]
use infra::InputProvider;
use infra::{run_day, FromFile, Literal};
use std::env;

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
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod infra;

fn main() {
    let day = env::args().nth(1).and_then(|d| d.parse::<u8>().ok());
    let times = env::args()
        .nth(2)
        .and_then(|d| d.parse::<u32>().ok())
        .unwrap_or(1);

    run_day(
        day,
        times,
        day01::Day1,
        (FromFile("day1.txt"), 956091),
        (FromFile("day1.txt"), 79734368),
    );

    run_day(
        day,
        times,
        day02::Day2,
        (FromFile("day2.txt"), 519),
        (FromFile("day2.txt"), 708),
    );

    run_day(
        day,
        times,
        day03::Day3,
        (FromFile("day3.txt"), 216),
        (FromFile("day3.txt"), 6708199680),
    );

    run_day(
        day,
        times,
        day04::Day4,
        (FromFile("day4.txt"), 250),
        (FromFile("day4.txt"), 158),
    );

    run_day(
        day,
        times,
        day05::Day5,
        (FromFile("day5.txt"), 832),
        (FromFile("day5.txt"), 517),
    );

    run_day(
        day,
        times,
        day06::Day6,
        (FromFile("day6.txt"), 6782),
        (FromFile("day6.txt"), 3596),
    );

    run_day(
        day,
        times,
        day07::Day7,
        (FromFile("day7.txt"), 128),
        (FromFile("day7.txt"), 20189),
    );

    run_day(
        day,
        times,
        day08::Day8,
        (FromFile("day8.txt"), 1814),
        (FromFile("day8.txt"), 1056),
    );

    run_day(
        day,
        times,
        day09::Day9,
        (FromFile("day9.txt"), 36845998),
        (FromFile("day9.txt"), 4830226),
    );

    run_day(
        day,
        times,
        day10::Day10,
        (FromFile("day10.txt"), 2263),
        (FromFile("day10.txt"), 396857386627072),
    );

    run_day(
        day,
        times,
        day11::Day11,
        (FromFile("day11.txt"), 2361),
        (FromFile("day11.txt"), 2119),
    );

    run_day(
        day,
        times,
        day12::Day12,
        (FromFile("day12.txt"), 381),
        (FromFile("day12.txt"), 28591),
    );

    run_day(
        day,
        times,
        day13::Day13,
        (FromFile("day13.txt"), 1915),
        (FromFile("day13.txt"), 294354277694107),
    );

    run_day(
        day,
        times,
        day14::Day14,
        (FromFile("day14.txt"), 12512013221615),
        (FromFile("day14.txt"), 3905642473893),
    );

    run_day(
        day,
        times,
        day15::Day15,
        (Literal(vec![0, 8, 15, 2, 12, 1, 4]), 289),
        (Literal(vec![0, 8, 15, 2, 12, 1, 4]), 1505722),
    );

    run_day(
        day,
        times,
        day16::Day16,
        (FromFile("day16.txt"), 28882),
        (FromFile("day16.txt"), 1429779530273),
    );

    run_day(
        day,
        times,
        day17::Day17,
        (FromFile("day17.txt"), 333),
        (FromFile("day17.txt"), 2676),
    );

    run_day(
        day,
        times,
        day18::Day18,
        (FromFile("day18.txt"), 12918250417632),
        (FromFile("day18.txt"), 171259538712010),
    );

    run_day(
        day,
        times,
        day19::Day19,
        (FromFile("day19.txt"), 226),
        (FromFile("day19.txt"), 355),
    );

    run_day(
        day,
        times,
        day20::Day20,
        (FromFile("day20.txt"), 17148689442341),
        (FromFile("day20.txt"), 2009),
    );

    run_day(
        day,
        times,
        day21::Day21,
        (FromFile("day21.txt"), 2874),
        (
            FromFile("day21.txt"),
            "gfvrr,ndkkq,jxcxh,bthjz,sgzr,mbkbn,pkkg,mjbtz".to_string(),
        ),
    );

    run_day(
        day,
        times,
        day22::Day22,
        (FromFile("day22.txt"), 31957),
        (FromFile("day22.txt"), 33212),
    );

    run_day(
        day,
        times,
        day23::Day23,
        (Literal("368195742"), "95648732".to_string()),
        (Literal("368195742"), 192515314252),
    );

    run_day(
        day,
        times,
        day24::Day24,
        (FromFile("day24.txt"), 356),
        (FromFile("day24.txt"), 3887),
    );
}
