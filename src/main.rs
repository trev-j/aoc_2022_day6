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
    const LOSS_OUTCOME: u32 = 1;
    const TIE_OUTCOME: u32 = 2;

    for line in input_file_contents.lines() {
        let line_symbols = line.split(" ");
        let symbols_vec: Vec<&str> = line_symbols.collect();
        let opponent_play: u32 = played_type_value(symbols_vec[0]);
        let mut my_play: u32 = opponent_play;
        let target_outcome: u32 = played_type_value(symbols_vec[1]);

        let mut round_outcome: RoundOutcome = RoundOutcome::Loss;
        
        if target_outcome == TIE_OUTCOME {
            round_outcome = RoundOutcome::Tie;
        } else if target_outcome == LOSS_OUTCOME {
            round_outcome = RoundOutcome::Loss;
            my_play = get_lose_move_value(&opponent_play);
        } else {
            round_outcome = RoundOutcome::Win;
            my_play = get_win_move_value(&opponent_play);
        }

        total_score += round_outcome_value(&round_outcome) + my_play;
    }

    println!("Total score: {}", total_score);
}

enum RoundOutcome {
    Win,
    Tie,
    Loss,
}

fn round_outcome_value(symbol: &RoundOutcome) -> u32 {
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

fn get_win_move_value(opponent_move: &u32) -> u32 {
    match opponent_move {
        1 => 2,
        2 => 3,
        3 => 1,
        &_ => 0,
    }
}

fn get_lose_move_value(opponent_move: &u32) -> u32 {
    match opponent_move {
        1 => 3,
        2 => 1,
        3 => 2,
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