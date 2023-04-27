# aws-iot-custom-auth-lambda

## Prereqs

1. Install git: https://git-scm.com/downloads
1. Clone with: https://github.com/TCROC/aws-iot-custom-auth-lambda.git
1. Install the rust toolset: https://www.rust-lang.org/tools/install
1. Install cargo lambda: https://github.com/awslabs/aws-lambda-rust-runtime

## How to build

```
cargo lambda build --arm64
```

You should then have a binary located at: 

```
target/lambda/bootstrap/bootstrap
```