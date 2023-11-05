use std::env;
use std::process;
use std::fs;

use aoc_2022_day6::find_marker_index;


fn main() {

    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let input_file_contents = fs::read_to_string(config.file_path).unwrap_or_else(|err| {
        eprintln!("Problem reading input file: {err}");
        process::exit(1);
    });

    let start_index = find_marker_index(&input_file_contents, 14);

    print!("\nStart index {}", start_index);
}

struct Config {
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();

        Ok(Config { file_path: file_path})
    }
}