mod secrets;

use lambda_http::{run, service_fn, Body, Error, Request, Response};
use secrets::SecretsError;
use serde::Serialize;

#[derive(Serialize)]
struct ParameterStoreResponse {
    value: String,
}

#[derive(Serialize)]
struct ParameterStoreErrorResponse {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

async fn handler(_event: Request) -> Result<Response<Body>, Error> {
    let (status, body) = match inner().await {
        Ok(o) => {
            let body_json = ParameterStoreResponse { value: o };
            (200, serde_json::to_string(&body_json)?) // Success { "value": "" }
        }
        Err(e) => {
            let body_json = ParameterStoreErrorResponse {
                message: e.to_string(),
            };
            (500, serde_json::to_string(&body_json)?) // Error { "message": "" }
        }
    };

    let response = Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(Body::from(body))?;

    Ok(response)
}

async fn inner() -> Result<String, SecretsError> {
    let secrets_client = secrets::Client::new();
    let value = secrets_client.get_parameter("/lambda", true).await?;

    Ok(value)
}
