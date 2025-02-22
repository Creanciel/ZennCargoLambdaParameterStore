import * as cdk from 'aws-cdk-lib';
import * as iam from 'aws-cdk-lib/aws-iam';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import * as logs from 'aws-cdk-lib/aws-logs';
import * as s3 from 'aws-cdk-lib/aws-s3';
import * as apigateway from 'aws-cdk-lib/aws-apigateway';
import type { Construct } from 'constructs';
import { getConfig } from './config';

const LAMBDA_EXTENSION_AWS_PARAMETERS_AND_SECRETS_US_EAST_1_ARM =
  'arn:aws:lambda:us-east-1:177933569100:layer:AWS-Parameters-and-Secrets-Lambda-Extension-Arm64:12';

export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const config = getConfig();

    const s3Bucket = s3.Bucket.fromBucketName(
      this,
      `${config.lambdaName}-s3-bucket`,
      config.lambdaImageBucket,
    );

    // Log Group
    const logGroup = new logs.LogGroup(
      this,
      `${config.lambdaName}-lambda-log-group`,
      {
        logGroupName: `/aws/lambda/${config.lambdaName}`,
        retention: logs.RetentionDays.TWO_WEEKS,
        removalPolicy: cdk.RemovalPolicy.DESTROY,
      },
    );

    // Role
    const role = new iam.Role(
      this,
      `${config.lambdaName}-lambda-role`,
      {
        roleName: `${config.lambdaName}-lambda-role`,
        assumedBy: new iam.ServicePrincipal('lambda.amazonaws.com'),
        description: `${config.lambdaName} lambda's role`,
      },
    );

    role.attachInlinePolicy(
      new iam.Policy(
        this,
        `${config.lambdaName}-lambda-default-policy`,
        {
          policyName: `${config.lambdaName}-lambda-default-policy`,
          statements: [
            new iam.PolicyStatement({
              effect: iam.Effect.ALLOW,
              actions: ['logs:CreateLogStream', 'logs:PutLogEvents'],
              resources: [logGroup.logGroupArn],
            }),
          ],
        },
      ),
    );

    role.attachInlinePolicy(
      new iam.Policy(
        this,
        `${config.lambdaName}-lambda-ssm-policy`,
        {
          policyName: `${config.lambdaName}-lambda-ssm-policy`,
          statements: [
            new iam.PolicyStatement({
              effect: iam.Effect.ALLOW,
              actions: ['ssm:GetParameter', 'kms:Decrypt'],
              resources: ['*'],
            }),
          ],
        },
      ),
    );

    const layers = [
      lambda.LayerVersion.fromLayerVersionArn(
        this,
        `${config.lambdaName}-lambda-extension-aws-parameters-secrets`,
        LAMBDA_EXTENSION_AWS_PARAMETERS_AND_SECRETS_US_EAST_1_ARM,
      ),
    ];

    const lmd = new lambda.Function(
      this,
      `${config.lambdaName}-lambda`,
      {
        runtime: lambda.Runtime.PROVIDED_AL2,
        handler: 'bootstrap',
        architecture: lambda.Architecture.ARM_64,
        code: lambda.Code.fromBucket(
          s3Bucket,
          config.lambdaImageKey,
        ),
        functionName: config.lambdaName,
        logGroup,
        role,
        memorySize: 128,
        timeout: cdk.Duration.seconds(30),
        layers,
      },
    );

    const integration = new apigateway.LambdaIntegration(lmd, {
      allowTestInvoke: false,
    });

    const api = new apigateway.RestApi(
      this,
      `${config.lambdaName}-api-gateway`,
      {
        restApiName: `${config.lambdaName}-api-gateway`,
        deploy: true,
        deployOptions: {
          stageName: 'main',
        },
      },
    );

    api.root.addMethod('GET', integration);
  }
}
