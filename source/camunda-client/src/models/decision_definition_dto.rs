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
pub struct DecisionDefinitionDto {
    /// The id of the decision definition
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The key of the decision definition, i.e., the id of the DMN 1.0 XML decision definition.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The category of the decision definition.
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The name of the decision definition.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The version of the decision definition that the engine assigned to it.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    /// The file name of the decision definition.
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// The deployment id of the decision definition.
    #[serde(rename = "deploymentId", skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// The tenant id of the decision definition.
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    /// The id of the decision requirements definition this decision definition belongs to.
    #[serde(rename = "decisionRequirementsDefinitionId", skip_serializing_if = "Option::is_none")]
    pub decision_requirements_definition_id: Option<String>,
    /// The key of the decision requirements definition this decision definition belongs to.
    #[serde(rename = "decisionRequirementsDefinitionKey", skip_serializing_if = "Option::is_none")]
    pub decision_requirements_definition_key: Option<String>,
    /// History time to live value of the decision definition. Is used within [History cleanup](https://docs.camunda.org/manual/7.14/user-guide/process-engine/history/#history-cleanup).
    #[serde(rename = "historyTimeToLive", skip_serializing_if = "Option::is_none")]
    pub history_time_to_live: Option<i32>,
    /// The version tag of the decision definition.
    #[serde(rename = "versionTag", skip_serializing_if = "Option::is_none")]
    pub version_tag: Option<String>,
}

impl DecisionDefinitionDto {
    pub fn new() -> DecisionDefinitionDto {
        DecisionDefinitionDto {
            id: None,
            key: None,
            category: None,
            name: None,
            version: None,
            resource: None,
            deployment_id: None,
            tenant_id: None,
            decision_requirements_definition_id: None,
            decision_requirements_definition_key: None,
            history_time_to_live: None,
            version_tag: None,
        }
    }
}

