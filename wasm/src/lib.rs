use wasm_bindgen::prelude::*;
use dice_roll::parser;

#[wasm_bindgen]
pub fn roll(input: &str) -> String {
    let result = match parser::parse(String::from(input)) {
        Ok(val) => val,
        Err(e) => {
            return e.to_string();
        }
    };

    let result = match  result.roll_dice() {
        Ok(val) => val,
        Err(e) => {
            return e.to_string();
        }
    };

    return result.to_string();
}
