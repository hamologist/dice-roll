import { RollPayload } from '../models/payload';
import { payloadProcessor } from './payload-processor';
import { Step } from '../models/response';

const validateRegex = new RegExp([
    /^ *[1-9][0-9]*d[1-9][0-9]* */,                               // Required first roll
    /([+-] *[1-9][0-9]* *)?/,                                     // Optional +- modifier on required first roll
    /((\+ *[1-9][0-9]*d[1-9][0-9]* *) *([+-] *[1-9][0-9]* *)?)*$/ // Optional rolls with single +- modifier
].map(r => r.source).join(''));
const diceRegex = /[1-9][0-9]*d[1-9][0-9]*/;
const delimRegex = /[+-]/g;

export const rollFromDiscordProcessor = (input: string) => {
    if (input.match(validateRegex)) {
        input = input.replace(/\s+/g, '');
        const rollPayload: RollPayload = {
            dice: [],
            count: 1,
        };
        const tokens = input.split(delimRegex);
        const delims = [...input.matchAll(delimRegex)];

        try {
            for (let i = 0; i < tokens.length; i++) {
                const token: string = tokens[i];

                if (token.match(diceRegex)) {
                    const diceToken = token.split(/d/);
                    rollPayload.dice.push({
                        count: Number(diceToken[0]),
                        sides: Number(diceToken[1]),
                    });
                } else {
                    rollPayload.dice[i - 1].modifier = Number(`${delims[i - 1]}${token}`)
                }
            }
        } catch (error) {
            return { content: 'Failed to build roll, make sure modifiers are at the end of the roll', flags: 1<<6 }
        }

        try {
            const rollResponse = payloadProcessor(rollPayload);
            const step: Step = rollResponse.step[0];
            const resultParts = [];

            for (let rolls of step.rolls) {
                let resultPart =  `(${rolls.rolls.join(') + (')})`;
                if (rolls.modifier) {
                    resultPart += rolls.modifier > 0 ? ` + ${rolls.modifier}` : ` - ${Math.abs(rolls.modifier)}`;
                }
                resultParts.push(resultPart);
            }
            return { content: `${resultParts.join(' + ')} = ${step.total}` }
        } catch {
            return { content: 'That roll is messed up...', flags: 1<<6 }
        }
    } else {
        return { content: 'Sorry bud, I don\'t know how to roll that...', flags: 1<<6 }
    }
};
