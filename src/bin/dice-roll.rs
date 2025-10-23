use dice_roll::{ Dice, RollRequest, roll_dice };
use serde_json;

fn main() {
    match roll_dice(RollRequest{
        dice: vec![Dice{
            count: 2,
            modifier: 1,
            sides: 4,
        }]
    }) {
        Ok(result) => {
            match serde_json::to_string_pretty(&result) {
                Ok(serialized) => println!("{}", serialized),
                Err(_) => println!("Failed to serialize response.")
            }
        },
        Err(err) => {
            println!("{}", err.to_string())
        }
    };
}
