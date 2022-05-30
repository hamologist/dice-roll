#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
import { DevStack } from '../lib/stacks/dev-stack';
import { ProdStack } from '../lib/stacks/prod-stack';

const app = new cdk.App();
new DevStack(app, 'DiceRollStack-Dev', {
    env: { account: process.env.CDK_DEFAULT_ACCOUNT, region: process.env.CDK_DEFAULT_REGION },
});
new ProdStack(app, 'DiceRollStack-Prod', {
  env: { account: process.env.CDK_DEFAULT_ACCOUNT, region: process.env.CDK_DEFAULT_REGION },
});
