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
    total: i32,
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
        let mut roll_response = RollResponse { rolls: Vec::new(), total: 0 };

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
            roll_response.total += rolls_total;
        }

        Ok(roll_response)
    }
}

impl RollResponse {
    pub fn to_string(self) -> String {
        let mut result = Vec::new();

        for rolls in self.rolls {
            for roll in rolls.rolls {
                if result.len() > 0 {
                    result.push("+".to_string())
                }
                result.push(format!("({} of {})", roll, rolls.sides));
            }
            if rolls.modifier != 0 {
                if rolls.modifier > 0 {
                    result.push("+".to_string());
                    result.push(rolls.modifier.to_string());
                } else {
                    result.push("-".to_string());
                    result.push(rolls.modifier.abs().to_string());
                }
            }
        }
        result.push("=".to_string());
        result.push(self.total.to_string());

        return result.join(" ");
    }

    pub fn to_json(self) -> Result<String, String> {
        match serde_json::to_string_pretty(&self) {
            Ok(serialized) => return Ok(serialized),
            Err(_) => return Err("Failed to serialize response.".to_string()),
        }
    }
}

pub mod parser {
    #[derive(Debug)]
    enum Tokens {
        Roll,
        PlusRollOrModifier,
        MinusModifier,
        Operator,
    }

    #[derive(Debug)]
    pub enum ParserErrors {
        RollParserError(RollTokenParserErrors),
        EmptyDiceForModifierError,
        ModifierParserError { token: String },
    }

    #[derive(Debug)]
    pub enum RollTokenParserErrors {
        DiceCountParserError { token: String },
        DiceSidesParserError { token: String },
    }

    pub fn parse(input: String) -> Result<crate::RollRequest, ParserErrors> {
        let input = input.as_bytes();
        let mut result = crate::RollRequest { dice: Vec::new() };
        let mut current_token_state = Tokens::Roll;
        let mut cursor = 0;
        let mut token = String::new();
        let input_end = input.len() - 1;
        while cursor < input_end + 1 {
            let byte = input[cursor];
            if byte == b' ' {
                cursor += 1;
                continue;
            }

            match current_token_state {
                Tokens::PlusRollOrModifier if byte == b'd' => {
                    current_token_state = Tokens::Roll;
                    token.push(byte as char);
                    cursor += 1;
                    continue;
                }
                Tokens::Roll | Tokens::PlusRollOrModifier | Tokens::MinusModifier => {
                    if cursor == input_end && byte != b'+' && byte != b'-' {
                        token.push(byte as char);
                    }

                    if cursor == input_end || byte == b'+' || byte == b'-' {
                        match current_token_state {
                            Tokens::Roll => {
                                let dice = match parse_roll_token(&token) {
                                    Ok(dice) => dice,
                                    Err(e) => {
                                        return Err(ParserErrors::RollParserError(e));
                                    }
                                };
                                result.dice.push(dice);
                                token = String::new();
                            }
                            Tokens::PlusRollOrModifier | Tokens::MinusModifier => {
                                let modifier: i32 = match token.parse() {
                                    Ok(val) => val,
                                    Err(_) => {
                                        return Err(ParserErrors::ModifierParserError { token });
                                    }
                                };
                                match result.dice.last_mut() {
                                    Some(dice) => {
                                        match current_token_state {
                                            Tokens::PlusRollOrModifier => {
                                                dice.modifier = modifier;
                                            },
                                            Tokens::MinusModifier => {
                                                dice.modifier = -modifier
                                            },
                                            _ => unreachable!()
                                        }
                                    }
                                    None => {
                                        return Err(ParserErrors::EmptyDiceForModifierError)
                                    }
                                }
                                token = String::new();
                            }
                            _ => unreachable!()
                        }
                        current_token_state = Tokens::Operator;

                        if cursor == input_end {
                            cursor += 1;
                        }
                    } else {
                        token.push(byte as char);
                        cursor += 1;
                    }
                    continue;
                }
                Tokens::Operator => {
                    match byte {
                        b'+' => {
                            current_token_state = Tokens::PlusRollOrModifier;
                        }
                        b'-' => {
                            current_token_state = Tokens::MinusModifier;
                        }
                        _ => unreachable!()
                    }
                    cursor += 1;
                    continue;
                }
            }
        }

        return Ok(result);
    }

    fn parse_roll_token(token: &String) -> Result<crate::Dice, RollTokenParserErrors> {
        let _token = token.as_bytes();
        let mut cursor = 0;

        // Parse the number of dice for the roll
        let mut fragment = String::new();
        while cursor < token.len() {
            let byte = _token[cursor];
            cursor += 1;

            if byte == b' ' {
                continue;
            } else if byte == b'd' {
                break;
            }

            fragment.push(byte as char);
        }
        let count: i32 = match fragment.parse() {
            Ok(num) => num,
            Err(_) => {
                return Err(RollTokenParserErrors::DiceCountParserError { token: token.clone() });
            }
        };

        // Parse the sides of the dice for the roll
        let mut fragment = String::new();
        while cursor < _token.len() {
            let byte = _token[cursor];
            cursor += 1;

            if byte == b' ' {
                continue;
            }

            fragment.push(byte as char);
        }
        let sides: i32 = match fragment.parse() {
            Ok(num) => num,
            Err(_) => {
                return Err(RollTokenParserErrors::DiceSidesParserError { token: token.clone() });
            }
        };

        let result = crate::Dice {
            count: count,
            sides: sides,
            modifier: 0,
        };

        return Ok(result);
    }

    impl ParserErrors {
        pub fn to_string(self) -> String {
            match self {
                Self::RollParserError(roll_token_parser_errors) => {
                    match roll_token_parser_errors {
                        RollTokenParserErrors::DiceCountParserError { token } => {
                            return format!("Invalid roll provided, {token}. Failed to parse dice count.");
                        },
                        RollTokenParserErrors::DiceSidesParserError { token } => {
                            return format!("Invalid roll provided, {token}. Failed to parse dice sides.");
                        }
                    }
                },
                Self::EmptyDiceForModifierError => {
                    return "No dice roll provided. Dice roll is in the form \"1d4\"".to_string();
                },
                Self::ModifierParserError { token } => {
                    return format!("Invalid modifier provided, {token}.");
                }
            }
        }
    }
}
