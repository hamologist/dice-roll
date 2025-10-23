use rand::prelude::*;
use serde::{Deserialize, Serialize};

struct BoundConstraint {
    lower_bound: i32,
    upper_bound: i32,
}

struct DiceConstraints {
    sides: BoundConstraint,
    modifier: BoundConstraint,
    count: BoundConstraint,
}

const DICE_CONSTRAINTS: DiceConstraints = DiceConstraints {
    sides: BoundConstraint {
        lower_bound: 1,
        upper_bound: 1000,
    },
    modifier: BoundConstraint {
        lower_bound: -100,
        upper_bound: 100,
    },
    count: BoundConstraint {
        lower_bound: 1,
        upper_bound: 100,
    },
};

pub enum RollRequestErrors {
    InvalidDiceSides { value: i32 },
    InvalidDiceModifier { value: i32 },
    InvalidDiceCount { value: i32 },
}

impl RollRequestErrors {
    pub fn to_string(self) -> String {
        match self {
            RollRequestErrors::InvalidDiceSides { value } => {
                format!(
                    "Dice sides must be between {} and {}, {} provided",
                    DICE_CONSTRAINTS.sides.lower_bound, DICE_CONSTRAINTS.sides.upper_bound, value
                )
            }
            RollRequestErrors::InvalidDiceModifier { value } => {
                format!(
                    "Dice modifier must be between {} and {}, {} provided",
                    DICE_CONSTRAINTS.modifier.lower_bound,
                    DICE_CONSTRAINTS.modifier.upper_bound,
                    value
                )
            }
            RollRequestErrors::InvalidDiceCount { value } => {
                format!(
                    "Dice count must be between {} and {}, {} provided",
                    DICE_CONSTRAINTS.count.lower_bound, DICE_CONSTRAINTS.count.upper_bound, value
                )
            }
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
    rolls: Vec<Rolls>,
}

pub fn roll_dice(roll_request: RollRequest) -> Result<RollResponse, RollRequestErrors> {
    let roll_request = match validate_roll_request(roll_request) {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    let mut rng = rand::rng();
    let mut roll_response = RollResponse { rolls: Vec::new() };

    for dice in roll_request.dice.iter() {
        let mut rolls = Vec::new();
        let mut rolls_total = dice.modifier;

        for _ in 0..dice.count {
            let roll = rng.random_range(1..=dice.sides);
            rolls.push(roll);
            rolls_total += roll
        }
        roll_response.rolls.push(Rolls {
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
        if dice.sides < DICE_CONSTRAINTS.sides.lower_bound
            || dice.sides > DICE_CONSTRAINTS.sides.upper_bound
        {
            return Err(RollRequestErrors::InvalidDiceSides { value: dice.sides });
        }
        if dice.modifier < DICE_CONSTRAINTS.modifier.lower_bound
            || dice.modifier > DICE_CONSTRAINTS.modifier.upper_bound
        {
            return Err(RollRequestErrors::InvalidDiceModifier {
                value: dice.modifier,
            });
        }
        if dice.count < DICE_CONSTRAINTS.count.lower_bound
            || dice.count > DICE_CONSTRAINTS.count.upper_bound
        {
            return Err(RollRequestErrors::InvalidDiceCount { value: dice.count });
        }
    }

    Ok(roll_request)
}
