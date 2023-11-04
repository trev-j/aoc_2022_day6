use std::env;
use std::process;
use std::fs;
use std::collections::HashMap;

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

    let mut start_index: usize = 0;
    let mut character_map: HashMap<char, bool> = HashMap::new();

    for (i, c) in input_file_contents.chars().enumerate() {
        if character_map.contains_key(&c) {
            character_map = HashMap::new(); // If character is already in map indicates repeat character and must restart map creation at current character
            character_map.insert(c, true);
        } else {
            character_map.insert(c, true);
        }

        // If count of keys in map is length of 4, all characters must be unique, and current index is start of sequence
        if character_map.keys().len() == 4 {
            start_index = i;
            break;
        }
    }

    print!("Start index {}, {:?}", start_index, character_map.keys());
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