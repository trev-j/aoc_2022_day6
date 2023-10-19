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

    let mut total_score: u32 = 0;
    const ROCK_VAL: u32 = 1;
    const PAPER_VAL: u32 = 2;
    const SCISSORS_VAL: u32 = 3;

    for line in input_file_contents.lines() {
        let line_symbols = line.split(" ");
        let symbols_vec: Vec<&str> = line_symbols.collect();
        let opponent_play: u32 = played_type_value(symbols_vec[0]);
        let my_play: u32 = played_type_value(symbols_vec[1]);
        let mut round_outcome: RoundOutcome = RoundOutcome::Loss;

        if(opponent_play == my_play) {
            round_outcome = RoundOutcome::Tie;
        } else if (opponent_play == ROCK_VAL && my_play == PAPER_VAL) {
            round_outcome = RoundOutcome::Win;
        } else if (opponent_play == PAPER_VAL && my_play == SCISSORS_VAL) {
            round_outcome = RoundOutcome::Win;
        } else if (opponent_play == SCISSORS_VAL && my_play == ROCK_VAL) {
            round_outcome = RoundOutcome::Win;
        } else {
            round_outcome = RoundOutcome::Loss;
        }

        total_score += round_outcome_value(round_outcome) + my_play;
    }

    println!("Total score: {}", total_score);
}

enum StrategySymbolKind {
    A,
    B,
    C,
    X,
    Y,
    Z,
}

enum RoundOutcome {
    Win,
    Tie,
    Loss,
}

fn symbol_win_value(symbol: StrategySymbolKind) -> u32 {
    match symbol {
        StrategySymbolKind::A => 1,
        StrategySymbolKind::B => 2,
        StrategySymbolKind::C => 3,
        StrategySymbolKind::X => 1,
        StrategySymbolKind::Y => 2,
        StrategySymbolKind::Z => 3,
    }
}

fn round_outcome_value(symbol: RoundOutcome) -> u32 {
    match symbol {
        RoundOutcome::Win => 6,
        RoundOutcome::Tie => 3,
        RoundOutcome::Loss => 0,
    }
}

fn played_type_value(played_type: &str) -> u32 {
    match played_type {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        &_ => 0,
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