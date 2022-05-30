import { Construct } from 'constructs';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import * as apigateway from 'aws-cdk-lib/aws-apigateway';

export interface DiceRollProps {
  scope: string;
  apiDeployOptions?: apigateway.StageOptions;
}

export class DiceRoll extends Construct {
  public readonly api: apigateway.RestApi;
  public readonly discordHandler: lambda.Function;
  public readonly restHandler: lambda.Function;

  constructor(scope: Construct, id: string, props: DiceRollProps) {
    super(scope, id);

    this.api = new apigateway.RestApi(this, 'DiceRollRestApi', {
      defaultCorsPreflightOptions: {
        allowMethods: [
          'OPTIONS',
          'POST'
        ],
        allowHeaders: [
          'Content-Type',
          'X-Amz-Date',
          'Authorization',
          'X-Api-Key',
          'X-Amz-Security-Token'
        ],
        allowOrigins: ['*']
      },
      deployOptions: props?.apiDeployOptions,
    })

    this.discordHandler = new lambda.Function(this, 'DiceRollDiscordHandler', {
      runtime: lambda.Runtime.NODEJS_14_X,
      code: lambda.Code.fromAsset('src/dice-roll', { exclude: ['*.ts'] }),
      handler: 'discord-webhook-handler.handler',
      memorySize: 256,
    });

    this.restHandler = new lambda.Function(this, 'DiceRollRestHandler', {
      runtime: lambda.Runtime.NODEJS_14_X,
      code: lambda.Code.fromAsset('src/dice-roll', { exclude: ['*.ts'] }),
      handler: 'rest-handler.handler',
      memorySize: 256,
    });

    const rollResource = this.api.root.addResource('roll');
    rollResource.addMethod('POST', new apigateway.LambdaIntegration(this.restHandler));
  }
}
