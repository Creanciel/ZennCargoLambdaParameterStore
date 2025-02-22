# ZennCardoLambdaParameterStore

Cargo Lambda から Lambda Extension を使って Parameter Store から値を取り出すサンプル

## How to use

0. Ready

	Docker と Node は最低限インストールしていること。 Cargo に関しては Docker 内でやるので不要

1. Build

	`/app/lambda` の Rust プロジェクトをビルド

	```sh
	./cargolambda build
	```

	`/app/target/lambda/lambda/bootstrap.zip` として出力される。

2. Upload

	前の zip ファイルを s3 にアップロードする。 CD などを作成するなら aws sdk で `s3 cp` などをするといい。

4. Deploy

	```sh
	export AWS_ACCOUNT='000000000000'               # AWS Account
	export LAMBDA_IMAGE_BUCKET='my-bucket'          # S3 Bucket 名
	export LAMBDA_IMAGE_KEY='image/bootstrap.zip'   # S3 キー
	./cargolambda cdk_deploy
	```



## Content

### `app`

Cargo Lambda のサンプル

### `cdk`

Lambda と API Gateway の作成をするための CDK。
Parameter Store は未設定。

### `docker`

Cargo Lambda ビルド用 Docker


## Reference

[Using Parameter Store parameters in AWS Lambda functions](https://docs.aws.amazon.com/systems-manager/latest/userguide/ps-integration-lambda-extensions.html)
