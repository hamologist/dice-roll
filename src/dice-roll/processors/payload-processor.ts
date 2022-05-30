import { RollResponse, Step } from '../models/response';
import { RollPayload } from '../models/payload';

export const payloadProcessor = (payload: RollPayload): RollResponse => {
    const rollResponse: RollResponse = {
        step: []
    };
    for (let rollCount = payload.count || 1; rollCount > 0; rollCount--) {
        const step: Step = {
            rolls: [],
            total: 0
        };
        for (const dice of payload.dice) {
            const diceCount = dice.count || 1;
            const diceModifier = dice.modifier || 0;
            const rolls: Array<number> = Array.from(
                {length: diceCount},
                () => Math.floor(Math.random() * dice.sides) + 1
            );
            const rollsTotal = rolls.reduce((a, b) => a + b, 0) + diceModifier;
            step.total += rollsTotal;
            step.rolls.push({
                count: diceCount,
                sides: dice.sides,
                modifier: diceModifier,
                rolls: rolls,
                total: rollsTotal
            });
        }
        rollResponse.step.push(step);
        rollCount--;
    }

    return rollResponse;
}
