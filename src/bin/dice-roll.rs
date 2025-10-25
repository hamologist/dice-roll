use clap::Parser;
use clap_stdin::FileOrStdin;
use dice_roll::{Dice, RollRequest};
use serde_json;


#[derive(Debug, Parser)]
struct Args {
    #[arg(default_value = "-")]
    input: FileOrStdin,
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


    match (RollRequest {
        dice: vec![Dice {
            count: 2,
            modifier: 1,
            sides: 4,
        }],
    }
    .roll_dice())
    {
        Ok(result) => match serde_json::to_string_pretty(&result) {
            Ok(serialized) => println!("{}", serialized),
            Err(_) => println!("Failed to serialize response."),
        },
        Err(err) => {
            println!("{}", err.to_string())
        }
    };
}
