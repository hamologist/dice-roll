use serde::{Serialize, Deserialize};
use rand::prelude::*;

pub enum RollRequestErrors {
    InvalidDiceSides,
    InvalidDiceModifier,
    InvalidDiceCount,
}

impl RollRequestErrors {
    pub fn to_string(self) -> String {
        match self {
            RollRequestErrors::InvalidDiceSides => {
                "Dice sides must be between 1 and 1000".to_string()
            }
            RollRequestErrors::InvalidDiceModifier => {
                "Dice modifier must be between -100 and 100".to_string()
            },
            RollRequestErrors::InvalidDiceCount => {
                "Dice count must be between 1 and 100".to_string()
            },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Dice {
    pub count: i32,
    pub sides: i32,
    pub modifier: i32,
}

#[derive(Deserialize, Debug)]
pub struct RollRequest {
    pub dice: Vec<Dice>,
}

#[derive(Serialize, Debug)]
struct Rolls {
    count: i32,
    sides: i32,
    modifier: i32,
    rolls: Vec<i32>,
    total: i32,
}

#[derive(Serialize, Debug)]
pub struct RollResponse {
    rolls: Vec<Rolls>
}

pub fn roll_dice(roll_request: RollRequest) -> Result<RollResponse, RollRequestErrors> {
    let roll_request = match validate_roll_request(roll_request) {
        Ok(ok) => ok,
        Err(err) => { return Err(err) }
    };

    let mut rng = rand::rng();
    let mut roll_response = RollResponse {
        rolls: Vec::new()
    };

    for dice in roll_request.dice.iter() {
        let mut rolls = Vec::new();
        let mut rolls_total = dice.modifier;

        for _ in 0..dice.count {
            let roll = rng.random_range(1..=dice.sides);
            rolls.push(roll);
            rolls_total += roll
        }
        roll_response.rolls.push(Rolls{
            count: dice.count,
            sides: dice.sides,
            modifier: dice.modifier,
            total: rolls_total,
            rolls,
        });
    }

    Ok(roll_response)
}

fn validate_roll_request(roll_request: RollRequest) -> Result<RollRequest, RollRequestErrors> {
    for dice in roll_request.dice.iter() {
        if dice.sides < 1 || dice.sides > 1000 {
            return Err(RollRequestErrors::InvalidDiceSides);
        }
        if dice.modifier < -100 || dice.modifier > 100 {
            return Err(RollRequestErrors::InvalidDiceModifier);
        }
        if dice.count < 1 || dice.count > 100 {
            return Err(RollRequestErrors::InvalidDiceCount);
        }
    }

    Ok(roll_request)
}
