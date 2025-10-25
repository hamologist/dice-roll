use rand::prelude::*;
use serde::{Deserialize, Serialize};


mod constraints {
    pub struct BoundConstraint {
        pub lower_bound: i32,
        pub upper_bound: i32,
    }

    pub mod dice {
        use crate::constraints::{BoundConstraint as BC};

        pub const SIDES: BC = BC {
            lower_bound: 1,
            upper_bound: 1000,
        };

        pub const MODIFIER: BC = BC {
            lower_bound: -100,
            upper_bound: 100,
        };

        pub const COUNT: BC = BC {
            lower_bound: 1,
            upper_bound: 100,
        };
    }
}

pub enum RollRequestErrors {
    InvalidDiceSides { value: i32 },
    InvalidDiceModifier { value: i32 },
    InvalidDiceCount { value: i32 },
}

impl RollRequestErrors {

    pub fn to_string(self) -> String {
        use crate::constraints::dice::{SIDES, MODIFIER, COUNT};

        match self {
            RollRequestErrors::InvalidDiceSides { value } => {
                format!(
                    "Dice sides must be between {} and {}, {} provided",
                    SIDES.lower_bound, SIDES.upper_bound, value
                )
            }
            RollRequestErrors::InvalidDiceModifier { value } => {
                format!(
                    "Dice modifier must be between {} and {}, {} provided",
                    MODIFIER.lower_bound,
                    MODIFIER.upper_bound,
                    value
                )
            }
            RollRequestErrors::InvalidDiceCount { value } => {
                format!(
                    "Dice count must be between {} and {}, {} provided",
                    COUNT.lower_bound, COUNT.upper_bound, value
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

impl RollRequest {
    fn validate_roll_request(roll_request: RollRequest) -> Result<RollRequest, RollRequestErrors> {
        use crate::constraints::dice::{SIDES, MODIFIER, COUNT};

        for dice in roll_request.dice.iter() {
            if dice.sides < SIDES.lower_bound
                || dice.sides > SIDES.upper_bound
            {
                return Err(RollRequestErrors::InvalidDiceSides { value: dice.sides });
            }
            if dice.modifier < MODIFIER.lower_bound
                || dice.modifier > MODIFIER.upper_bound
            {
                return Err(RollRequestErrors::InvalidDiceModifier {
                    value: dice.modifier,
                });
            }
            if dice.count < COUNT.lower_bound
                || dice.count > COUNT.upper_bound
            {
                return Err(RollRequestErrors::InvalidDiceCount { value: dice.count });
            }
        }

        Ok(roll_request)
    }

    pub fn roll_dice(self) -> Result<RollResponse, RollRequestErrors> {
        let roll_request = match RollRequest::validate_roll_request(self) {
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
}
