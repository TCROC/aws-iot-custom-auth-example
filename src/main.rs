use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, str};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(func)).await?;
    Ok(())
}

async fn func(event: LambdaEvent<IotAuthEvent>) -> Result<AwsAuthResponse, Error> {
    let topic_root = env::var("LS_AWS_TOPIC_ROOT")?;
    let aws_account_id = env::var("LS_AWS_ACCOUNT_ID")?;
    let aws_region = env::var("LS_AWS_REGION")?; // example: us-east-1
    let aws_password = env::var("LS_AWS_PASSWORD")?;
    let disconnect_secs = env::var("LS_AWS_DISCONNECT_AFTER_IN_SECONDS")?.parse::<u32>()?;
    let refresh_secs = env::var("LS_AWS_REFRESH_AFTER_IN_SECONDS")?.parse::<u32>()?;

    let mqtt_password_vu8 = base64::decode(event.payload.protocol_data.mqtt.password)?;
    let mqtt_password_str = str::from_utf8(&mqtt_password_vu8)?;

    if aws_password != mqtt_password_str {
        return Ok(AwsAuthResponse::auth_false());
    }

    let arn = format!("arn:aws:iot:{aws_region}:{aws_account_id}");
    let client_id = event.payload.protocol_data.mqtt.client_id;

    Ok(AwsAuthResponse {
        is_authenticated: true,
        principal_id: client_id.to_string(),
        disconnect_after_in_seconds: disconnect_secs,
        refresh_after_in_seconds: refresh_secs,
        policy_documents: vec![AwsPolicyDocument {
            version: "2012-10-17".to_string(),
            statement: vec![
                // Allow users to connect with their user id as the client id
                AwsPolicyDocumentStatement {
                    effect: "Allow".to_string(),
                    action: vec!["iot:Connect".to_string()],
                    resource: vec![format!("{arn}:client/${{iot:ClientId}}")],
                    condition: None, /* Some(HashMap::from([(
                        "ArnEquals".to_string(),
                        HashMap::from([(
                            "iot:LastWillTopic".to_string(),
                            format!("{arn}:topic/{topic_root}/s/${{iot:ClientId}}"),
                        )]),
                    )])),*/
                },
                // Allow users to receive messages from this root topic
                AwsPolicyDocumentStatement {
                    effect: "Allow".to_string(),
                    action: vec!["iot:Receive".to_string()],
                    resource: vec![format!("{arn}:topic/{topic_root}/*")],
                    condition: None,
                },
                AwsPolicyDocumentStatement {
                    effect: "Allow".to_string(),
                    action: vec!["iot:Publish".to_string()],
                    resource: vec![
                        // Allows users to publish direct messages to other users
                        format!("{arn}:topic/{topic_root}/d/*/${{iot:ClientId}}"),
                        // Allows users to publish to a party
                        format!("{arn}:topic/{topic_root}/p/*/${{iot:ClientId}}"),
                        // Allows users to publish to their subscribers
                        format!("{arn}:topic/{topic_root}/s/${{iot:ClientId}}"),
                    ],
                    condition: None,
                },
                AwsPolicyDocumentStatement {
                    effect: "Allow".to_string(),
                    action: vec!["iot:Subscribe".to_string()],
                    resource: vec![
                        // Allows users to subscribe to their direct messages
                        format!("{arn}:topicfilter/{topic_root}/d/${{iot:ClientId}}/*"),
                        // Allows users to subscribe to users in a party
                        format!("{arn}:topicfilter/{topic_root}/p/*/*"),
                        // Allow users to subscribe to other users
                        format!("{arn}:topicfilter/{topic_root}/s/*"),
                        // Allow users to subscribe to flexmatch ticket updates
                        format!("{arn}:topicfilter/{topic_root}/f/*"),
                    ],
                    condition: None,
                },
            ],
        }],
    })
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct IotAuthEvent {
    protocol_data: IotAuthEventProtocolData,
}

#[derive(Deserialize, Clone)]
struct IotAuthEventProtocolData {
    mqtt: IotAuthEventMqtt,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct IotAuthEventMqtt {
    password: String,
    client_id: String
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AwsAuthResponse {
    is_authenticated: bool, //A Boolean that determines whether client can connect.
    principal_id: String,   //A string that identifies the connection in logs.
    disconnect_after_in_seconds: u32,
    refresh_after_in_seconds: u32,
    policy_documents: Vec<AwsPolicyDocument>,
}

impl AwsAuthResponse {
    fn auth_false() -> AwsAuthResponse {
        AwsAuthResponse {
            is_authenticated: false,
            principal_id: String::default(),
            disconnect_after_in_seconds: 0,
            refresh_after_in_seconds: 0,
            policy_documents: Vec::default(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct AwsPolicyDocument {
    version: String,
    statement: Vec<AwsPolicyDocumentStatement>,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct AwsPolicyDocumentStatement {
    effect: String,
    action: Vec<String>,
    resource: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<HashMap<String, HashMap<String, String>>>,
}
