/*
 * Camunda BPM REST API
 *
 * OpenApi Spec for Camunda BPM REST API.
 *
 * The version of the OpenAPI document: 7.14.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use reqwest;

use super::{Error, configuration};

pub struct HistoricProcessInstanceApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl HistoricProcessInstanceApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> HistoricProcessInstanceApiClient {
        HistoricProcessInstanceApiClient {
            configuration,
        }
    }
}

pub trait HistoricProcessInstanceApi {
    fn delete_historic_process_instance(&self, id: &str, fail_if_not_exists: Option<bool>) -> Result<(), Error>;
    fn delete_historic_process_instances_async(&self, delete_historic_process_instances_dto: Option<crate::models::DeleteHistoricProcessInstancesDto>) -> Result<crate::models::BatchDto, Error>;
    fn delete_historic_variable_instances_of_historic_process_instance(&self, id: &str) -> Result<(), Error>;
    fn get_historic_process_instance(&self, id: &str) -> Result<crate::models::HistoricProcessInstanceDto, Error>;
    //fn get_historic_process_instance_duration_report(&self, report_type: &str, period_unit: &str, process_definition_id_in: Option<&str>, process_definition_key_in: Option<&str>, started_before: Option<String>, started_after: Option<String>) -> Result<crate::models::AnyType, Error>;
    fn get_historic_process_instances(&self, sort_by: Option<&str>, sort_order: Option<&str>, first_result: Option<i32>, max_results: Option<i32>, process_instance_id: Option<&str>, process_instance_ids: Option<&str>, process_definition_id: Option<&str>, process_definition_key: Option<&str>, process_definition_key_in: Option<&str>, process_definition_name: Option<&str>, process_definition_name_like: Option<&str>, process_definition_key_not_in: Option<&str>, process_instance_business_key: Option<&str>, process_instance_business_key_like: Option<&str>, root_process_instances: Option<bool>, finished: Option<bool>, unfinished: Option<bool>, with_incidents: Option<bool>, with_root_incidents: Option<bool>, incident_type: Option<&str>, incident_status: Option<&str>, incident_message: Option<&str>, incident_message_like: Option<&str>, started_before: Option<String>, started_after: Option<String>, finished_before: Option<String>, finished_after: Option<String>, executed_activity_after: Option<String>, executed_activity_before: Option<String>, executed_job_after: Option<String>, executed_job_before: Option<String>, started_by: Option<&str>, super_process_instance_id: Option<&str>, sub_process_instance_id: Option<&str>, super_case_instance_id: Option<&str>, sub_case_instance_id: Option<&str>, case_instance_id: Option<&str>, tenant_id_in: Option<&str>, without_tenant_id: Option<bool>, executed_activity_id_in: Option<&str>, active_activity_id_in: Option<&str>, active: Option<bool>, suspended: Option<bool>, completed: Option<bool>, externally_terminated: Option<bool>, internally_terminated: Option<bool>, variables: Option<&str>, variable_names_ignore_case: Option<bool>, variable_values_ignore_case: Option<bool>) -> Result<Vec<crate::models::HistoricProcessInstanceDto>, Error>;
    fn get_historic_process_instances_count(&self, process_instance_id: Option<&str>, process_instance_ids: Option<&str>, process_definition_id: Option<&str>, process_definition_key: Option<&str>, process_definition_key_in: Option<&str>, process_definition_name: Option<&str>, process_definition_name_like: Option<&str>, process_definition_key_not_in: Option<&str>, process_instance_business_key: Option<&str>, process_instance_business_key_like: Option<&str>, root_process_instances: Option<bool>, finished: Option<bool>, unfinished: Option<bool>, with_incidents: Option<bool>, with_root_incidents: Option<bool>, incident_type: Option<&str>, incident_status: Option<&str>, incident_message: Option<&str>, incident_message_like: Option<&str>, started_before: Option<String>, started_after: Option<String>, finished_before: Option<String>, finished_after: Option<String>, executed_activity_after: Option<String>, executed_activity_before: Option<String>, executed_job_after: Option<String>, executed_job_before: Option<String>, started_by: Option<&str>, super_process_instance_id: Option<&str>, sub_process_instance_id: Option<&str>, super_case_instance_id: Option<&str>, sub_case_instance_id: Option<&str>, case_instance_id: Option<&str>, tenant_id_in: Option<&str>, without_tenant_id: Option<bool>, executed_activity_id_in: Option<&str>, active_activity_id_in: Option<&str>, active: Option<bool>, suspended: Option<bool>, completed: Option<bool>, externally_terminated: Option<bool>, internally_terminated: Option<bool>, variables: Option<&str>, variable_names_ignore_case: Option<bool>, variable_values_ignore_case: Option<bool>) -> Result<crate::models::CountResultDto, Error>;
    fn query_historic_process_instances(&self, first_result: Option<i32>, max_results: Option<i32>, historic_process_instance_query_dto: Option<crate::models::HistoricProcessInstanceQueryDto>) -> Result<Vec<crate::models::HistoricProcessInstanceDto>, Error>;
    fn query_historic_process_instances_count(&self, historic_process_instance_query_dto: Option<crate::models::HistoricProcessInstanceQueryDto>) -> Result<crate::models::CountResultDto, Error>;
    fn set_removal_time_async(&self, set_removal_time_to_historic_process_instances_dto: Option<crate::models::SetRemovalTimeToHistoricProcessInstancesDto>) -> Result<crate::models::BatchDto, Error>;
}

impl HistoricProcessInstanceApi for HistoricProcessInstanceApiClient {
    fn delete_historic_process_instance(&self, id: &str, fail_if_not_exists: Option<bool>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/history/process-instance/{id}", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref s) = fail_if_not_exists {
            req_builder = req_builder.query(&[("failIfNotExists", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn delete_historic_process_instances_async(&self, delete_historic_process_instances_dto: Option<crate::models::DeleteHistoricProcessInstancesDto>) -> Result<crate::models::BatchDto, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/history/process-instance/delete", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&delete_historic_process_instances_dto);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn delete_historic_variable_instances_of_historic_process_instance(&self, id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/history/process-instance/{id}/variable-instances", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn get_historic_process_instance(&self, id: &str) -> Result<crate::models::HistoricProcessInstanceDto, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/history/process-instance/{id}", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }
/*
    fn get_historic_process_instance_duration_report(&self, report_type: &str, period_unit: &str, process_definition_id_in: Option<&str>, process_definition_key_in: Option<&str>, started_before: Option<String>, started_after: Option<String>) -> Result<crate::models::AnyType, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/history/process-instance/report", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("reportType", &report_type.to_string())]);
        req_builder = req_builder.query(&[("periodUnit", &period_unit.to_string())]);
        if let Some(ref s) = process_definition_id_in {
            req_builder = req_builder.query(&[("processDefinitionIdIn", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_key_in {
            req_builder = req_builder.query(&[("processDefinitionKeyIn", &s.to_string())]);
        }
        if let Some(ref s) = started_before {
            req_builder = req_builder.query(&[("startedBefore", &s.to_string())]);
        }
        if let Some(ref s) = started_after {
            req_builder = req_builder.query(&[("startedAfter", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }
*/
    fn get_historic_process_instances(&self, sort_by: Option<&str>, sort_order: Option<&str>, first_result: Option<i32>, max_results: Option<i32>, process_instance_id: Option<&str>, process_instance_ids: Option<&str>, process_definition_id: Option<&str>, process_definition_key: Option<&str>, process_definition_key_in: Option<&str>, process_definition_name: Option<&str>, process_definition_name_like: Option<&str>, process_definition_key_not_in: Option<&str>, process_instance_business_key: Option<&str>, process_instance_business_key_like: Option<&str>, root_process_instances: Option<bool>, finished: Option<bool>, unfinished: Option<bool>, with_incidents: Option<bool>, with_root_incidents: Option<bool>, incident_type: Option<&str>, incident_status: Option<&str>, incident_message: Option<&str>, incident_message_like: Option<&str>, started_before: Option<String>, started_after: Option<String>, finished_before: Option<String>, finished_after: Option<String>, executed_activity_after: Option<String>, executed_activity_before: Option<String>, executed_job_after: Option<String>, executed_job_before: Option<String>, started_by: Option<&str>, super_process_instance_id: Option<&str>, sub_process_instance_id: Option<&str>, super_case_instance_id: Option<&str>, sub_case_instance_id: Option<&str>, case_instance_id: Option<&str>, tenant_id_in: Option<&str>, without_tenant_id: Option<bool>, executed_activity_id_in: Option<&str>, active_activity_id_in: Option<&str>, active: Option<bool>, suspended: Option<bool>, completed: Option<bool>, externally_terminated: Option<bool>, internally_terminated: Option<bool>, variables: Option<&str>, variable_names_ignore_case: Option<bool>, variable_values_ignore_case: Option<bool>) -> Result<Vec<crate::models::HistoricProcessInstanceDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/history/process-instance", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = sort_by {
            req_builder = req_builder.query(&[("sortBy", &s.to_string())]);
        }
        if let Some(ref s) = sort_order {
            req_builder = req_builder.query(&[("sortOrder", &s.to_string())]);
        }
        if let Some(ref s) = first_result {
            req_builder = req_builder.query(&[("firstResult", &s.to_string())]);
        }
        if let Some(ref s) = max_results {
            req_builder = req_builder.query(&[("maxResults", &s.to_string())]);
        }
        if let Some(ref s) = process_instance_id {
            req_builder = req_builder.query(&[("processInstanceId", &s.to_string())]);
        }
        if let Some(ref s) = process_instance_ids {
            req_builder = req_builder.query(&[("processInstanceIds", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_id {
            req_builder = req_builder.query(&[("processDefinitionId", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_key {
            req_builder = req_builder.query(&[("processDefinitionKey", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_key_in {
            req_builder = req_builder.query(&[("processDefinitionKeyIn", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_name {
            req_builder = req_builder.query(&[("processDefinitionName", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_name_like {
            req_builder = req_builder.query(&[("processDefinitionNameLike", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_key_not_in {
            req_builder = req_builder.query(&[("processDefinitionKeyNotIn", &s.to_string())]);
        }
        if let Some(ref s) = process_instance_business_key {
            req_builder = req_builder.query(&[("processInstanceBusinessKey", &s.to_string())]);
        }
        if let Some(ref s) = process_instance_business_key_like {
            req_builder = req_builder.query(&[("processInstanceBusinessKeyLike", &s.to_string())]);
        }
        if let Some(ref s) = root_process_instances {
            req_builder = req_builder.query(&[("rootProcessInstances", &s.to_string())]);
        }
        if let Some(ref s) = finished {
            req_builder = req_builder.query(&[("finished", &s.to_string())]);
        }
        if let Some(ref s) = unfinished {
            req_builder = req_builder.query(&[("unfinished", &s.to_string())]);
        }
        if let Some(ref s) = with_incidents {
            req_builder = req_builder.query(&[("withIncidents", &s.to_string())]);
        }
        if let Some(ref s) = with_root_incidents {
            req_builder = req_builder.query(&[("withRootIncidents", &s.to_string())]);
        }
        if let Some(ref s) = incident_type {
            req_builder = req_builder.query(&[("incidentType", &s.to_string())]);
        }
        if let Some(ref s) = incident_status {
            req_builder = req_builder.query(&[("incidentStatus", &s.to_string())]);
        }
        if let Some(ref s) = incident_message {
            req_builder = req_builder.query(&[("incidentMessage", &s.to_string())]);
        }
        if let Some(ref s) = incident_message_like {
            req_builder = req_builder.query(&[("incidentMessageLike", &s.to_string())]);
        }
        if let Some(ref s) = started_before {
            req_builder = req_builder.query(&[("startedBefore", &s.to_string())]);
        }
        if let Some(ref s) = started_after {
            req_builder = req_builder.query(&[("startedAfter", &s.to_string())]);
        }
        if let Some(ref s) = finished_before {
            req_builder = req_builder.query(&[("finishedBefore", &s.to_string())]);
        }
        if let Some(ref s) = finished_after {
            req_builder = req_builder.query(&[("finishedAfter", &s.to_string())]);
        }
        if let Some(ref s) = executed_activity_after {
            req_builder = req_builder.query(&[("executedActivityAfter", &s.to_string())]);
        }
        if let Some(ref s) = executed_activity_before {
            req_builder = req_builder.query(&[("executedActivityBefore", &s.to_string())]);
        }
        if let Some(ref s) = executed_job_after {
            req_builder = req_builder.query(&[("executedJobAfter", &s.to_string())]);
        }
        if let Some(ref s) = executed_job_before {
            req_builder = req_builder.query(&[("executedJobBefore", &s.to_string())]);
        }
        if let Some(ref s) = started_by {
            req_builder = req_builder.query(&[("startedBy", &s.to_string())]);
        }
        if let Some(ref s) = super_process_instance_id {
            req_builder = req_builder.query(&[("superProcessInstanceId", &s.to_string())]);
        }
        if let Some(ref s) = sub_process_instance_id {
            req_builder = req_builder.query(&[("subProcessInstanceId", &s.to_string())]);
        }
        if let Some(ref s) = super_case_instance_id {
            req_builder = req_builder.query(&[("superCaseInstanceId", &s.to_string())]);
        }
        if let Some(ref s) = sub_case_instance_id {
            req_builder = req_builder.query(&[("subCaseInstanceId", &s.to_string())]);
        }
        if let Some(ref s) = case_instance_id {
            req_builder = req_builder.query(&[("caseInstanceId", &s.to_string())]);
        }
        if let Some(ref s) = tenant_id_in {
            req_builder = req_builder.query(&[("tenantIdIn", &s.to_string())]);
        }
        if let Some(ref s) = without_tenant_id {
            req_builder = req_builder.query(&[("withoutTenantId", &s.to_string())]);
        }
        if let Some(ref s) = executed_activity_id_in {
            req_builder = req_builder.query(&[("executedActivityIdIn", &s.to_string())]);
        }
        if let Some(ref s) = active_activity_id_in {
            req_builder = req_builder.query(&[("activeActivityIdIn", &s.to_string())]);
        }
        if let Some(ref s) = active {
            req_builder = req_builder.query(&[("active", &s.to_string())]);
        }
        if let Some(ref s) = suspended {
            req_builder = req_builder.query(&[("suspended", &s.to_string())]);
        }
        if let Some(ref s) = completed {
            req_builder = req_builder.query(&[("completed", &s.to_string())]);
        }
        if let Some(ref s) = externally_terminated {
            req_builder = req_builder.query(&[("externallyTerminated", &s.to_string())]);
        }
        if let Some(ref s) = internally_terminated {
            req_builder = req_builder.query(&[("internallyTerminated", &s.to_string())]);
        }
        if let Some(ref s) = variables {
            req_builder = req_builder.query(&[("variables", &s.to_string())]);
        }
        if let Some(ref s) = variable_names_ignore_case {
            req_builder = req_builder.query(&[("variableNamesIgnoreCase", &s.to_string())]);
        }
        if let Some(ref s) = variable_values_ignore_case {
            req_builder = req_builder.query(&[("variableValuesIgnoreCase", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_historic_process_instances_count(&self, process_instance_id: Option<&str>, process_instance_ids: Option<&str>, process_definition_id: Option<&str>, process_definition_key: Option<&str>, process_definition_key_in: Option<&str>, process_definition_name: Option<&str>, process_definition_name_like: Option<&str>, process_definition_key_not_in: Option<&str>, process_instance_business_key: Option<&str>, process_instance_business_key_like: Option<&str>, root_process_instances: Option<bool>, finished: Option<bool>, unfinished: Option<bool>, with_incidents: Option<bool>, with_root_incidents: Option<bool>, incident_type: Option<&str>, incident_status: Option<&str>, incident_message: Option<&str>, incident_message_like: Option<&str>, started_before: Option<String>, started_after: Option<String>, finished_before: Option<String>, finished_after: Option<String>, executed_activity_after: Option<String>, executed_activity_before: Option<String>, executed_job_after: Option<String>, executed_job_before: Option<String>, started_by: Option<&str>, super_process_instance_id: Option<&str>, sub_process_instance_id: Option<&str>, super_case_instance_id: Option<&str>, sub_case_instance_id: Option<&str>, case_instance_id: Option<&str>, tenant_id_in: Option<&str>, without_tenant_id: Option<bool>, executed_activity_id_in: Option<&str>, active_activity_id_in: Option<&str>, active: Option<bool>, suspended: Option<bool>, completed: Option<bool>, externally_terminated: Option<bool>, internally_terminated: Option<bool>, variables: Option<&str>, variable_names_ignore_case: Option<bool>, variable_values_ignore_case: Option<bool>) -> Result<crate::models::CountResultDto, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/history/process-instance/count", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = process_instance_id {
            req_builder = req_builder.query(&[("processInstanceId", &s.to_string())]);
        }
        if let Some(ref s) = process_instance_ids {
            req_builder = req_builder.query(&[("processInstanceIds", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_id {
            req_builder = req_builder.query(&[("processDefinitionId", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_key {
            req_builder = req_builder.query(&[("processDefinitionKey", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_key_in {
            req_builder = req_builder.query(&[("processDefinitionKeyIn", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_name {
            req_builder = req_builder.query(&[("processDefinitionName", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_name_like {
            req_builder = req_builder.query(&[("processDefinitionNameLike", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_key_not_in {
            req_builder = req_builder.query(&[("processDefinitionKeyNotIn", &s.to_string())]);
        }
        if let Some(ref s) = process_instance_business_key {
            req_builder = req_builder.query(&[("processInstanceBusinessKey", &s.to_string())]);
        }
        if let Some(ref s) = process_instance_business_key_like {
            req_builder = req_builder.query(&[("processInstanceBusinessKeyLike", &s.to_string())]);
        }
        if let Some(ref s) = root_process_instances {
            req_builder = req_builder.query(&[("rootProcessInstances", &s.to_string())]);
        }
        if let Some(ref s) = finished {
            req_builder = req_builder.query(&[("finished", &s.to_string())]);
        }
        if let Some(ref s) = unfinished {
            req_builder = req_builder.query(&[("unfinished", &s.to_string())]);
        }
        if let Some(ref s) = with_incidents {
            req_builder = req_builder.query(&[("withIncidents", &s.to_string())]);
        }
        if let Some(ref s) = with_root_incidents {
            req_builder = req_builder.query(&[("withRootIncidents", &s.to_string())]);
        }
        if let Some(ref s) = incident_type {
            req_builder = req_builder.query(&[("incidentType", &s.to_string())]);
        }
        if let Some(ref s) = incident_status {
            req_builder = req_builder.query(&[("incidentStatus", &s.to_string())]);
        }
        if let Some(ref s) = incident_message {
            req_builder = req_builder.query(&[("incidentMessage", &s.to_string())]);
        }
        if let Some(ref s) = incident_message_like {
            req_builder = req_builder.query(&[("incidentMessageLike", &s.to_string())]);
        }
        if let Some(ref s) = started_before {
            req_builder = req_builder.query(&[("startedBefore", &s.to_string())]);
        }
        if let Some(ref s) = started_after {
            req_builder = req_builder.query(&[("startedAfter", &s.to_string())]);
        }
        if let Some(ref s) = finished_before {
            req_builder = req_builder.query(&[("finishedBefore", &s.to_string())]);
        }
        if let Some(ref s) = finished_after {
            req_builder = req_builder.query(&[("finishedAfter", &s.to_string())]);
        }
        if let Some(ref s) = executed_activity_after {
            req_builder = req_builder.query(&[("executedActivityAfter", &s.to_string())]);
        }
        if let Some(ref s) = executed_activity_before {
            req_builder = req_builder.query(&[("executedActivityBefore", &s.to_string())]);
        }
        if let Some(ref s) = executed_job_after {
            req_builder = req_builder.query(&[("executedJobAfter", &s.to_string())]);
        }
        if let Some(ref s) = executed_job_before {
            req_builder = req_builder.query(&[("executedJobBefore", &s.to_string())]);
        }
        if let Some(ref s) = started_by {
            req_builder = req_builder.query(&[("startedBy", &s.to_string())]);
        }
        if let Some(ref s) = super_process_instance_id {
            req_builder = req_builder.query(&[("superProcessInstanceId", &s.to_string())]);
        }
        if let Some(ref s) = sub_process_instance_id {
            req_builder = req_builder.query(&[("subProcessInstanceId", &s.to_string())]);
        }
        if let Some(ref s) = super_case_instance_id {
            req_builder = req_builder.query(&[("superCaseInstanceId", &s.to_string())]);
        }
        if let Some(ref s) = sub_case_instance_id {
            req_builder = req_builder.query(&[("subCaseInstanceId", &s.to_string())]);
        }
        if let Some(ref s) = case_instance_id {
            req_builder = req_builder.query(&[("caseInstanceId", &s.to_string())]);
        }
        if let Some(ref s) = tenant_id_in {
            req_builder = req_builder.query(&[("tenantIdIn", &s.to_string())]);
        }
        if let Some(ref s) = without_tenant_id {
            req_builder = req_builder.query(&[("withoutTenantId", &s.to_string())]);
        }
        if let Some(ref s) = executed_activity_id_in {
            req_builder = req_builder.query(&[("executedActivityIdIn", &s.to_string())]);
        }
        if let Some(ref s) = active_activity_id_in {
            req_builder = req_builder.query(&[("activeActivityIdIn", &s.to_string())]);
        }
        if let Some(ref s) = active {
            req_builder = req_builder.query(&[("active", &s.to_string())]);
        }
        if let Some(ref s) = suspended {
            req_builder = req_builder.query(&[("suspended", &s.to_string())]);
        }
        if let Some(ref s) = completed {
            req_builder = req_builder.query(&[("completed", &s.to_string())]);
        }
        if let Some(ref s) = externally_terminated {
            req_builder = req_builder.query(&[("externallyTerminated", &s.to_string())]);
        }
        if let Some(ref s) = internally_terminated {
            req_builder = req_builder.query(&[("internallyTerminated", &s.to_string())]);
        }
        if let Some(ref s) = variables {
            req_builder = req_builder.query(&[("variables", &s.to_string())]);
        }
        if let Some(ref s) = variable_names_ignore_case {
            req_builder = req_builder.query(&[("variableNamesIgnoreCase", &s.to_string())]);
        }
        if let Some(ref s) = variable_values_ignore_case {
            req_builder = req_builder.query(&[("variableValuesIgnoreCase", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn query_historic_process_instances(&self, first_result: Option<i32>, max_results: Option<i32>, historic_process_instance_query_dto: Option<crate::models::HistoricProcessInstanceQueryDto>) -> Result<Vec<crate::models::HistoricProcessInstanceDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/history/process-instance", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref s) = first_result {
            req_builder = req_builder.query(&[("firstResult", &s.to_string())]);
        }
        if let Some(ref s) = max_results {
            req_builder = req_builder.query(&[("maxResults", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&historic_process_instance_query_dto);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn query_historic_process_instances_count(&self, historic_process_instance_query_dto: Option<crate::models::HistoricProcessInstanceQueryDto>) -> Result<crate::models::CountResultDto, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/history/process-instance/count", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&historic_process_instance_query_dto);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn set_removal_time_async(&self, set_removal_time_to_historic_process_instances_dto: Option<crate::models::SetRemovalTimeToHistoricProcessInstancesDto>) -> Result<crate::models::BatchDto, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/history/process-instance/set-removal-time", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&set_removal_time_to_historic_process_instances_dto);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

}
