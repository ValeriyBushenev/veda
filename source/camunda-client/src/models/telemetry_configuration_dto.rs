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
pub struct TelemetryConfigurationDto {
    /// Specifies if the telemetry data should be sent or not.
    #[serde(rename = "enableTelemetry", skip_serializing_if = "Option::is_none")]
    pub enable_telemetry: Option<bool>,
}

impl TelemetryConfigurationDto {
    pub fn new() -> TelemetryConfigurationDto {
        TelemetryConfigurationDto {
            enable_telemetry: None,
        }
    }
}


