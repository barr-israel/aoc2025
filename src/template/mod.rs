use std::{env, fs};

pub mod aoc_cli;
pub mod commands;
pub mod runner;

pub use day::*;

mod day;
mod readme_benchmarks;
mod run_multi;
mod timings;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

/// Helper function that reads a text file to a string.
#[must_use]
pub fn read_file(folder: &str, day: Day) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("data").join(folder).join(format!("{day}.txt"));
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

/// Helper function that reads a text file to string, appending a part suffix. E.g. like `01-2.txt`.
#[must_use]
pub fn read_file_part(folder: &str, day: Day, part: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd
        .join("data")
        .join(folder)
        .join(format!("{day}-{part}.txt"));
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

/// Creates the constant `DAY` and sets up the input and runner for each part.
///
/// The optional, second parameter (1 or 2) allows you to only run a single part of the solution.
#[macro_export]
macro_rules! solution {
    ($day:expr) => {
        $crate::solution!(@impl $day, [part_one,part_one_tui,part_two,part_two_tui]);
    };

    (@impl $day:expr, [$func1:expr,$func1_tui:expr,$func2:expr,$func2_tui:expr]) => {
        /// The current day.
        const DAY: $crate::template::Day = $crate::day!($day);

        #[cfg(feature = "dhat-heap")]
        #[global_allocator]
        static ALLOC: dhat::Alloc = dhat::Alloc;

        fn main() {
            use std::{env,process};
            use $crate::template::runner::*;
            let input = $crate::template::read_file("inputs", DAY);
            let args: Vec<String> = env::args().collect();
            if args.contains(&"--tui".into()) {
                let part_index = args.iter().position(|x| x == "--part").expect("part number expected") + 1;
                match args[part_index].parse::<u8>(){
                    Err(_)=>{
                        eprintln!("Unexpected command-line input. Format: cargo solve 1 --submit 1");
                        process::exit(1);
                    },
                    Ok(1)=> $func1_tui(&input).unwrap(),
                    Ok(2)=> $func2_tui(&input).unwrap(),
                    _=>{
                        eprintln!("Part must be 1 or 2");
                        process::exit(1);
                    }
                };

            }else{
                run_part($func1, &input, DAY,1);
                run_part($func2, &input, DAY,2);
        }
        }
    };
}
