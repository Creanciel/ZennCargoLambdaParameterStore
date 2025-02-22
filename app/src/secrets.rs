use serde::Deserialize;

#[cfg(not(test))]
#[allow(unused)]
pub type Client = SecretsClient;
#[cfg(test)]
#[allow(unused)]
pub type Client = SecretsMockClient;

#[derive(Debug)]
#[allow(unused)]
pub enum SecretsError {
    AwsSessionTokenNotFound,
    ReqwestError(reqwest::Error),
    ParameterEmpty,
}

impl std::fmt::Display for SecretsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SecretsError::AwsSessionTokenNotFound => write!(f, "AWS session token not found"),
            SecretsError::ReqwestError(err) => {
                write!(f, "Reqwest error: {}", err)
            }
            SecretsError::ParameterEmpty => write!(f, "Parameter is empty"),
        }
    }
}

#[allow(unused)]
pub struct SecretsClient {
    reqwest_client: reqwest::Client,
}

#[allow(unused)]
pub struct SecretsMockClient;

impl SecretsClient {
    #[allow(unused)]
    pub fn new() -> Self {
        let reqwest_client = reqwest::Client::new();

        Self { reqwest_client }
    }

    #[allow(unused)]
    pub async fn get_parameter(
        &self,
        key: &str,
        secure_string: bool,
    ) -> Result<String, SecretsError> {
        let aws_session_token = std::env::var("AWS_SESSION_TOKEN")
            .map_err(|_| SecretsError::AwsSessionTokenNotFound)?;
        let parameters_secrets_extension_http_port =
            std::env::var("PARAMETERS_SECRETS_EXTENSION_HTTP_PORT").unwrap_or("2773".to_owned());

        let endpoint = format!(
            "http://localhost:{}/systemsmanager/parameters/get/",
            parameters_secrets_extension_http_port
        );

        let mut query = vec![("name", key)];
        if secure_string {
            query.push(("withDecryption", "true"));
        }

        #[derive(Deserialize)]
        struct SsmResponseParameter {
            #[serde(rename = "Value")]
            value: String,
        }

        #[derive(Deserialize)]
        struct SsmResponse {
            #[serde(rename = "Parameter")]
            parameter: SsmResponseParameter,
        }

        let response: SsmResponse = self
            .reqwest_client
            .get(&endpoint)
            .header("X-Aws-Parameters-Secrets-Token", &aws_session_token)
            .query(&query)
            .send()
            .await
            .map_err(SecretsError::ReqwestError)?
            .json()
            .await
            .map_err(SecretsError::ReqwestError)?;

        let value = response.parameter.value;

        if value.is_empty() {
            Err(SecretsError::ParameterEmpty)
        } else {
            Ok(value)
        }
    }
}

impl SecretsMockClient {
    #[allow(unused)]
    pub fn new() -> Self {
        Self
    }

    #[allow(unused)]
    pub async fn get_parameter(
        &self,
        key: &str,
        secure_string: bool,
    ) -> Result<String, SecretsError> {
        Ok(key.to_string())
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn get_paramter_test() {
        use super::Client;

        let client = Client::new();

        let key = "key";

        let value = client.get_parameter(key, true).await.unwrap();

        // Mock はすべて入力した key を パラメーターストア の値として返してくるようになっている
        assert_eq!(value, key);
    }
}
