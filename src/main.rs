use std::env;
use std::process;
use std::fs;

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

    // Starting crate stacks
    //     [H]         [H]         [V]    
    //     [V]         [V] [J]     [F] [F]
    //     [S] [L]     [M] [B]     [L] [J]
    //     [C] [N] [B] [W] [D]     [D] [M]
    // [G] [L] [M] [S] [S] [C]     [T] [V]
    // [P] [B] [B] [P] [Q] [S] [L] [H] [B]
    // [N] [J] [D] [V] [C] [Q] [Q] [M] [P]
    // [R] [T] [T] [R] [G] [W] [F] [W] [L]
    //  1   2   3   4   5   6   7   8   9 

    let mut crate_stacks: Vec<CrateStack> = vec![
        CrateStack::new(vec!['R', 'N', 'P', 'G']),
        CrateStack::new(vec!['T', 'J', 'B', 'L', 'C', 'S', 'V', 'H']),
        CrateStack::new(vec!['T', 'D', 'B', 'M', 'N', 'L']),
        CrateStack::new(vec!['R', 'V', 'P', 'S', 'B']),
        CrateStack::new(vec!['G', 'C', 'Q', 'S', 'W', 'M', 'V', 'H']),
        CrateStack::new(vec!['W', 'Q', 'S', 'C', 'D', 'B', 'J']),
        CrateStack::new(vec!['F', 'Q', 'L']),
        CrateStack::new(vec!['W', 'M', 'H', 'T', 'D', 'L', 'F', 'V']),
        CrateStack::new(vec!['L', 'P', 'B', 'V', 'M', 'J', 'F']),
    ];

    for line in input_file_contents.lines() {
        let instructions_line = line.split(" ");
        let instructions_line_vec: Vec<&str> = instructions_line.collect();
        let move_qty: usize = instructions_line_vec[1].parse::<usize>().unwrap();
        let move_from: usize = instructions_line_vec[3].parse::<usize>().unwrap() - 1; // subtract one to get index in crate_stacks array
        let move_to: usize = instructions_line_vec[5].parse::<usize>().unwrap() - 1;

        for n in 0..move_qty {
            let moved_crate: char = crate_stacks[move_from].pop().unwrap();
            crate_stacks[move_to].push(moved_crate);
        }
    }

    let mut top_crates: &str = "";
    for n in 0..crate_stacks.len() {
        print!("{}", crate_stacks[n].pop().unwrap());
    }
}

struct CrateStack {
    crates: Vec<char>
}

impl CrateStack {
    fn new(crates: Vec<char>) -> CrateStack {

        CrateStack {
            crates: crates
        }
    }

    fn pop(&mut self) -> Option<char> {
        self.crates.pop()
    }

    fn push(&mut self, value: char) {
        self.crates.push(value);
    }

    fn top_crate(&self) -> Option<char> {
        if (self.crates.len() < 1) {
            return None;
        } else {
            return Some(self.crates[self.crates.len() - 1]);
        }
    }
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