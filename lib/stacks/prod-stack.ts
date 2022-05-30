import { Stack, StackProps } from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { DiceRoll } from '../dice-roll';
import * as ssm from 'aws-cdk-lib/aws-ssm';
import * as certificatemanager from 'aws-cdk-lib/aws-certificatemanager';
import * as route53 from 'aws-cdk-lib/aws-route53';
import * as route53Targets from 'aws-cdk-lib/aws-route53-targets';

export class ProdStack extends Stack {
    public constructor(scope: Construct, id: string, props?: StackProps) {
        super(scope, id, props);

        const diceRoll = new DiceRoll(this, 'DiceRoll', {
            scope: 'prod',
        });
        const rootDomain = ssm.StringParameter.valueFromLookup(this, '/dice-roll/prod/root-domain');
        const certificate = certificatemanager.Certificate.fromCertificateArn(
            this,
            'Certificate',
            ssm.StringParameter.valueForStringParameter(this, '/dice-roll/prod/certificate-arn'),
        );
        const zone = route53.HostedZone.fromLookup(this, 'BaseZone', {
            domainName: rootDomain,
        });
        diceRoll.api.addDomainName('DomainName', {
            domainName: `dice-roll.${rootDomain}`,
            certificate,
        });
        new route53.ARecord(this, 'ApiDNS', {
            zone,
            recordName: 'dice-roll',
            target: route53.RecordTarget.fromAlias(
                new route53Targets.ApiGateway(diceRoll.api),
            ),
        });

        new ssm.StringParameter(this, 'DiscordHandlerARNStringParameter', {
            parameterName: '/dice-roll/prod/discord-handler-arn',
            stringValue: diceRoll.discordHandler.functionArn,
        });
    }
}
