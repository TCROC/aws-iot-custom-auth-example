# aws-iot-custom-auth-lambda

## Prereqs

1. Install git: https://git-scm.com/downloads
1. Install the rust toolset: https://www.rust-lang.org/tools/install
1. Install cargo lambda: https://github.com/awslabs/aws-lambda-rust-runtime
1. Clone:
    ```
    git clone https://github.com/TCROC/aws-iot-custom-auth-lambda.git
    ```

## How to build

```
cargo lambda build --release --arm64
```

You should then have a binary located at: 

```
target/lambda/bootstrap/bootstrap
```

or you can build a zipped version that is ready to be uploaded to aws with

```
cargo lambda build --release --arm64
```

and the zip will be located at:

```
target/lambda/bootstrap/bootstrap.zip
```

## AWS Configuration

1. Upload lambda: https://docs.aws.amazon.com/sdk-for-rust/latest/dg/lambda.html#lambda-step3
1. Connect iot: https://docs.aws.amazon.com/iot/latest/developerguide/config-custom-auth.html#custom-auth-create-authorizer
    - NOTE: Those docs only have cli instructions. There is also a web console here: https://us-east-1.console.aws.amazon.com/iot/home?region=us-east-1#/authorizerhub