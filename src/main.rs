mod client;
mod util;

use crate::client::Client;
use clap::{App, Arg};
use std::fmt::Display;
use std::str::FromStr;
use std::time::Instant;

fn main() {
    let matches = App::new("aoc")
        .version("0.1")
        .about("Execute Advent of Code problems")
        .author("Nicholas Lordello")
        .arg(
            Arg::with_name("year")
                .short("y")
                .long("year")
                .value_name("YEAR")
                .default_value("2023")
                .takes_value(true)
                .validator(validate::<i32>),
        )
        .arg(
            Arg::with_name("show-time")
                .long("show-time")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("days")
                .short("d")
                .value_name("DAY")
                .required(true)
                .multiple(true)
                .validator(validate::<i32>),
        )
        .get_matches();

    let year: i32 = matches.value_of("year").unwrap().parse().unwrap();
    let show_time = matches.is_present("show-time");
    let days: Vec<i32> = matches
        .values_of("days")
        .unwrap()
        .map(|d| d.parse::<i32>().unwrap())
        .collect();

    let client = Client::from_env().expect("failed to create adventofcode.com client");
    for day in days {
        let input = client
            .get_input(year, day)
            .unwrap_or_else(|_| panic!("failed to get input for {} day {}", year, day));

        let start = Instant::now();
        let answers = solve(year, day, &input);
        let time = Instant::now() - start;

        if show_time {
            let time = time.as_secs() as f64 + f64::from(time.subsec_nanos()) * 1e-9;
            println!("Day {} ({:.2}s)", day, time);
        } else {
            println!("Day {}", day);
        }

        println!("  puzzle 1: {}", answers.0);
        println!("  puzzle 2: {}", answers.1);
    }
}

fn validate<V: FromStr>(s: String) -> Result<(), String>
where
    V: FromStr,
    V::Err: Display,
{
    match V::from_str(&s) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

macro_rules! advent {
    ($(
        $year:tt {
            $($day:tt,)*
        },
    )*) => {
        $(
            mod $year {
                $(
                    pub mod $day;
                )*
            }
        )*

        fn solve(year: i32, day: i32, input: &str) -> (String, String) {
            let year_str = format!("year{}", year);
            let day_str = format!("day{:02}", day);

            match (year_str.as_str(), day_str.as_str()) {
                $(
                    $(
                        (stringify!($year), stringify!($day)) => (
                            $year::$day::puzzle1(input).to_string(),
                            $year::$day::puzzle2(input).to_string(),
                        ),
                    )*
                )*
                _ => panic!("failed to get input for year {} day {}", year, day),
            }
        }
    };
}

advent!(
    year2023 {
        day01,
        day02,
        day03,
        day04,
        day05,
        day06,
        day07,
        day08,
        day09,
        day10,
        //poop
    },
    year2022 {
        day01,
        day02,
        day03,
        day04,
        day05,
        day06,
        day07,
        day08,
        day09,
        day10,
    },
    year2020 {
        day01,
        //day02,
    },
    year2019 {
        day01,
        day02,
        day03,
        day04,
        day05,
    },
    year2018 {
        day01,
        //        day02,
    },
);
