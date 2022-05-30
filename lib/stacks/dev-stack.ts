import { Stack, StackProps } from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { DiceRoll } from '../dice-roll';
import * as ssm from 'aws-cdk-lib/aws-ssm';

export class DevStack extends Stack {
    constructor(scope: Construct, id: string, props?: StackProps) {
        super(scope, id, props);

        const diceRoll = new DiceRoll(this, 'DiceRoll', {
            apiDeployOptions: {
                stageName: 'dev',
            },
            scope: 'dev',
        });

        new ssm.StringParameter(this, 'DiscordHandlerARNStringParameter', {
            parameterName: '/dice-roll/dev/discord-handler-arn',
            stringValue: diceRoll.discordHandler.functionArn,
        });
    }
}
