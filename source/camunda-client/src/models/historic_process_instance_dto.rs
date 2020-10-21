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
pub struct HistoricProcessInstanceDto {
    /// The id of the process instance.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The process instance id of the root process instance that initiated the process.
    #[serde(rename = "rootProcessInstanceId", skip_serializing_if = "Option::is_none")]
    pub root_process_instance_id: Option<String>,
    /// The id of the parent process instance, if it exists.
    #[serde(rename = "superProcessInstanceId", skip_serializing_if = "Option::is_none")]
    pub super_process_instance_id: Option<String>,
    /// The id of the parent case instance, if it exists.
    #[serde(rename = "superCaseInstanceId", skip_serializing_if = "Option::is_none")]
    pub super_case_instance_id: Option<String>,
    /// The id of the parent case instance, if it exists.
    #[serde(rename = "caseInstanceId", skip_serializing_if = "Option::is_none")]
    pub case_instance_id: Option<String>,
    /// The name of the process definition that this process instance belongs to.
    #[serde(rename = "processDefinitionName", skip_serializing_if = "Option::is_none")]
    pub process_definition_name: Option<String>,
    /// The key of the process definition that this process instance belongs to.
    #[serde(rename = "processDefinitionKey", skip_serializing_if = "Option::is_none")]
    pub process_definition_key: Option<String>,
    /// The version of the process definition that this process instance belongs to.
    #[serde(rename = "processDefinitionVersion", skip_serializing_if = "Option::is_none")]
    pub process_definition_version: Option<i32>,
    /// The id of the process definition that this process instance belongs to.
    #[serde(rename = "processDefinitionId", skip_serializing_if = "Option::is_none")]
    pub process_definition_id: Option<String>,
    /// The business key of the process instance.
    #[serde(rename = "businessKey", skip_serializing_if = "Option::is_none")]
    pub business_key: Option<String>,
    /// The time the instance was started. Default [format](https://docs.camunda.org/manual/7.14/reference/rest/overview/date-format/) `yyyy-MM-dd'T'HH:mm:ss.SSSZ`.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// The time the instance ended. Default [format](https://docs.camunda.org/manual/7.14/reference/rest/overview/date-format/) `yyyy-MM-dd'T'HH:mm:ss.SSSZ`.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// The time after which the instance should be removed by the History Cleanup job. Default [format](https://docs.camunda.org/manual/7.14/reference/rest/overview/date-format/) `yyyy-MM-dd'T'HH:mm:ss.SSSZ`.
    #[serde(rename = "removalTime", skip_serializing_if = "Option::is_none")]
    pub removal_time: Option<String>,
    /// The time the instance took to finish (in milliseconds).
    #[serde(rename = "durationInMillis", skip_serializing_if = "Option::is_none")]
    pub duration_in_millis: Option<i32>,
    /// The id of the user who started the process instance.
    #[serde(rename = "startUserId", skip_serializing_if = "Option::is_none")]
    pub start_user_id: Option<String>,
    /// The id of the initial activity that was executed (e.g., a start event).
    #[serde(rename = "startActivityId", skip_serializing_if = "Option::is_none")]
    pub start_activity_id: Option<String>,
    /// The provided delete reason in case the process instance was canceled during execution.
    #[serde(rename = "deleteReason", skip_serializing_if = "Option::is_none")]
    pub delete_reason: Option<String>,
    /// The tenant id of the process instance.
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    /// Last state of the process instance, possible values are:  `ACTIVE` - running process instance  `SUSPENDED` - suspended process instances  `COMPLETED` - completed through normal end event  `EXTERNALLY_TERMINATED` - terminated externally, for instance through REST API  `INTERNALLY_TERMINATED` - terminated internally, for instance by terminating boundary event
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
}

impl HistoricProcessInstanceDto {
    pub fn new() -> HistoricProcessInstanceDto {
        HistoricProcessInstanceDto {
            id: None,
            root_process_instance_id: None,
            super_process_instance_id: None,
            super_case_instance_id: None,
            case_instance_id: None,
            process_definition_name: None,
            process_definition_key: None,
            process_definition_version: None,
            process_definition_id: None,
            business_key: None,
            start_time: None,
            end_time: None,
            removal_time: None,
            duration_in_millis: None,
            start_user_id: None,
            start_activity_id: None,
            delete_reason: None,
            tenant_id: None,
            state: None,
        }
    }
}

/// Last state of the process instance, possible values are:  `ACTIVE` - running process instance  `SUSPENDED` - suspended process instances  `COMPLETED` - completed through normal end event  `EXTERNALLY_TERMINATED` - terminated externally, for instance through REST API  `INTERNALLY_TERMINATED` - terminated internally, for instance by terminating boundary event
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "ACTIVE")]
    ACTIVE,
    #[serde(rename = "SUSPENDED")]
    SUSPENDED,
    #[serde(rename = "COMPLETED")]
    COMPLETED,
    #[serde(rename = "EXTERNALLY_TERMINATED")]
    EXTERNALLYTERMINATED,
    #[serde(rename = "INTERNALLY_TERMINATED")]
    INTERNALLYTERMINATED,
}
