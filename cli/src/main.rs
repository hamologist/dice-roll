use std::fs;
use std::io::{self, Read};

use clap::ArgAction;
use dice_roll::parser;

struct CommandContext {
    as_json: bool,
    input: String,
}

enum InitClapErrors {
    FailedToOpenFileError,
    FailedToReadFromStdin,
    FailedToReadFromFile,
}

fn init_clap() -> Result<CommandContext, InitClapErrors> {
    let matches = clap::Command::new("dice-roll")
        .about("Simulates dice rolls")
        .arg(
            clap::Arg::new("as_json")
                .long("as-json")
                .action(ArgAction::SetTrue)
                .help("Changes output to JSON."),
        )
        .arg(
            clap::Arg::new("file")
                .default_value("-")
                .action(ArgAction::Set)
                .help("Reads input from the provided file or STDIN if no value is provided"),
        )
        .get_matches();

    let filename = matches.get_one::<String>("file").unwrap();
    let mut input_reader: Box<dyn Read> = match filename {
        _ if filename == "-" => Box::new(io::stdin()),
        _ => match fs::File::open(filename) {
            Ok(file) => Box::new(file),
            Err(_) => {
                return Err(InitClapErrors::FailedToOpenFileError);
            }
        },
    };

    let mut input: String = String::new();
    match input_reader.read_to_string(&mut input) {
        Ok(_) => {}
        Err(_) => {
            return Err(match filename {
                _ if filename == "-" => InitClapErrors::FailedToReadFromStdin,
                _ => InitClapErrors::FailedToReadFromFile,
            });
        }
    };

    return Ok(CommandContext {
        as_json: *matches.get_one::<bool>("as_json").unwrap(),
        input: input.trim().to_string(),
    });
}

fn main() {
    let command_context = match init_clap() {
        Ok(context) => context,
        Err(e) => {
            match e {
                InitClapErrors::FailedToOpenFileError => {
                    println!("Failed to open provided file.")
                },
                InitClapErrors::FailedToReadFromStdin => {
                    println!("Failed to read input from STDIN.")
                }
                InitClapErrors::FailedToReadFromFile => {
                    println!("Failed to read input from file.")
                }
            }
            return;
        }
    };

    let roll_request = match parser::parse(command_context.input) {
        Ok(roll_request) => roll_request,
        Err(e) => {
            println!("{}", e.to_string());
            return;
        }
    };
    let result = match roll_request.roll_dice() {
        Ok(result) => result,
        Err(e) => {
            println!("{}", e.to_string());
            return;
        }
    };

    match command_context.as_json {
        true => {
            match serde_json::to_string_pretty(&result.to_json()) {
                Ok(serialized) => {
                    println!("{}", serialized)
                }
                Err(_) => {
                    println!("Failed to serialize RollResponse into JSON.");
                    return;
                }
            };
        }
        false => {
            println!("{}", result.to_string());
        }
    }
}
