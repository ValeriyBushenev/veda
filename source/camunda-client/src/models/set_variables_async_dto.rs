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
pub struct SetVariablesAsyncDto {
    /// A list of process instance ids that define a group of process instances to which the operation will set variables.  Please note that if `processInstanceIds`, `processInstanceQuery` and `historicProcessInstanceQuery` are defined, the resulting operation will be performed on the union of these sets.
    #[serde(rename = "processInstanceIds", skip_serializing_if = "Option::is_none")]
    pub process_instance_ids: Option<Vec<String>>,
    #[serde(rename = "processInstanceQuery", skip_serializing_if = "Option::is_none")]
    pub process_instance_query: Option<crate::models::ProcessInstanceQueryDto>,
    #[serde(rename = "historicProcessInstanceQuery", skip_serializing_if = "Option::is_none")]
    pub historic_process_instance_query: Option<crate::models::HistoricProcessInstanceQueryDto>,
    /// A variables the operation will set in the root scope of the process instances.
    #[serde(rename = "variables", skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, crate::models::VariableValueDto>>,
}

impl SetVariablesAsyncDto {
    pub fn new() -> SetVariablesAsyncDto {
        SetVariablesAsyncDto {
            process_instance_ids: None,
            process_instance_query: None,
            historic_process_instance_query: None,
            variables: None,
        }
    }
}


