import Fastify from 'fastify';
import { z } from 'zod';

type RollResponse = {
    step: Array<Step>
};

type Step = {
    rolls: Array<Roll>,
    total: number
};

type Roll = {
    count: number,
    sides: number,
    modifier: number,
    rolls: Array<number>,
    total: number
};

const fastify = Fastify({
  logger: true
});

const payloadSchema = z.object({
  count: z.number()
    .positive()
    .lt(100),
  dice: z.object({
    count: z.number()
      .positive()
      .lt(100),
    sides: z.number()
      .positive()
      .lt(1000),
    modifier: z.number()
  }).array()
});

fastify.post('/', async (request) => {
  const payload = payloadSchema.parse(request.body);

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
});

try {
  await fastify.listen({ port: 3000 })
} catch (err) {
  fastify.log.error(err)
  process.exit(1)
}
