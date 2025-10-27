use clap::Parser;
use clap_stdin::FileOrStdin;
use dice_roll::{parser};


#[derive(Debug, Parser)]
struct Args {
    #[arg(default_value = "-")]
    input: FileOrStdin,

    #[clap(long, action)]
    as_json: bool,
}

fn main() {
    let args = Args::parse();
    let input = match args.input.contents() {
        Ok(value) => value,
        Err(_) => {
            println!("Failed to read input.");
            return;
        }
    };


    let roll_request = match parser::parse(input) {
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

    match args.as_json {
        true => {
            match serde_json::to_string_pretty(&result.to_json()) {
                Ok(serialized) => {
                    println!("{}", serialized)
                },
                Err(_) => {
                    println!("Failed to serialize RollResponse into JSON.");
                    return;
                }
            };
        },
        false => {
            println!("{}", result.to_string());
        }
    }
}
