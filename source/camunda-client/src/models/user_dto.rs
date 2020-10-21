/*
 * Camunda BPM REST API
 *
 * OpenApi Spec for Camunda BPM REST API.
 *
 * The version of the OpenAPI document: 7.14.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserDto {
    #[serde(rename = "profile", skip_serializing_if = "Option::is_none")]
    pub profile: Option<crate::models::UserProfileDto>,
    #[serde(rename = "credentials", skip_serializing_if = "Option::is_none")]
    pub credentials: Option<crate::models::UserCredentialsDto>,
}

impl UserDto {
    pub fn new() -> UserDto {
        UserDto {
            profile: None,
            credentials: None,
        }
    }
}

