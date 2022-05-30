export interface RollPayload {
    dice: Array<Dice>,
    count?: number
}

export interface Dice {
    count?: number,
    sides: number,
    modifier?: number
}

const isPositiveInteger = (num: number): boolean => {
    return Number.isInteger(num) && num > 0;
};

export const isRollPayload = (obj: any): obj is RollPayload => {
    const count = obj.count;
    const diceData: Array<Dice> = obj.dice;

    if (count !== undefined && (!isPositiveInteger(count) || count > 100)) {
        return false;
    }

    if (!diceData || !Array.isArray(diceData) || diceData.length > 100) {
        return false;
    }

    for (const diceDatum of diceData) {
        const diceCount = diceDatum.count;
        const diceSides = diceDatum.sides;
        const diceModifier = diceDatum.modifier;

        if (
            (diceCount !== undefined && (!isPositiveInteger(diceCount) || diceCount > 100)) ||
            (!isPositiveInteger(diceSides) || diceSides > 1000) ||
            (diceModifier !== undefined && !Number.isInteger(diceModifier))
        ) {
            return false;
        }
    }

    return true;
};
