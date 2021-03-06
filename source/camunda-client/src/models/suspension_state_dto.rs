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
pub struct SuspensionStateDto {
    /// A Boolean value which indicates whether to activate or suspend a given process instance. When the value is set to true, the given process instance will be suspended and when the value is set to false, the given process instance will be activated.
    #[serde(rename = "suspended", skip_serializing_if = "Option::is_none")]
    pub suspended: Option<bool>,
}

impl SuspensionStateDto {
    pub fn new() -> SuspensionStateDto {
        SuspensionStateDto {
            suspended: None,
        }
    }
}


