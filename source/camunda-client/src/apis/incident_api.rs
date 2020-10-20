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

pub struct IncidentApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl IncidentApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> IncidentApiClient {
        IncidentApiClient {
            configuration,
        }
    }
}

pub trait IncidentApi {
    fn get_incident(&self, id: &str) -> Result<crate::models::IncidentDto, Error>;
    fn get_incidents(&self, incident_id: Option<&str>, incident_type: Option<&str>, incident_message: Option<&str>, incident_message_like: Option<&str>, process_definition_id: Option<&str>, process_definition_key_in: Option<&str>, process_instance_id: Option<&str>, execution_id: Option<&str>, incident_timestamp_before: Option<String>, incident_timestamp_after: Option<String>, activity_id: Option<&str>, failed_activity_id: Option<&str>, cause_incident_id: Option<&str>, root_cause_incident_id: Option<&str>, configuration: Option<&str>, tenant_id_in: Option<&str>, job_definition_id_in: Option<&str>, sort_by: Option<&str>, sort_order: Option<&str>) -> Result<Vec<crate::models::IncidentDto>, Error>;
    fn get_incidents_count(&self, incident_id: Option<&str>, incident_type: Option<&str>, incident_message: Option<&str>, incident_message_like: Option<&str>, process_definition_id: Option<&str>, process_definition_key_in: Option<&str>, process_instance_id: Option<&str>, execution_id: Option<&str>, incident_timestamp_before: Option<String>, incident_timestamp_after: Option<String>, activity_id: Option<&str>, failed_activity_id: Option<&str>, cause_incident_id: Option<&str>, root_cause_incident_id: Option<&str>, configuration: Option<&str>, tenant_id_in: Option<&str>, job_definition_id_in: Option<&str>) -> Result<Vec<crate::models::CountResultDto>, Error>;
    fn resolve_incident(&self, id: &str) -> Result<(), Error>;
}

impl IncidentApi for IncidentApiClient {
    fn get_incident(&self, id: &str) -> Result<crate::models::IncidentDto, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/incident/{id}", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_incidents(&self, incident_id: Option<&str>, incident_type: Option<&str>, incident_message: Option<&str>, incident_message_like: Option<&str>, process_definition_id: Option<&str>, process_definition_key_in: Option<&str>, process_instance_id: Option<&str>, execution_id: Option<&str>, incident_timestamp_before: Option<String>, incident_timestamp_after: Option<String>, activity_id: Option<&str>, failed_activity_id: Option<&str>, cause_incident_id: Option<&str>, root_cause_incident_id: Option<&str>, _configuration: Option<&str>, tenant_id_in: Option<&str>, job_definition_id_in: Option<&str>, sort_by: Option<&str>, sort_order: Option<&str>) -> Result<Vec<crate::models::IncidentDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/incident", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = incident_id {
            req_builder = req_builder.query(&[("incidentId", &s.to_string())]);
        }
        if let Some(ref s) = incident_type {
            req_builder = req_builder.query(&[("incidentType", &s.to_string())]);
        }
        if let Some(ref s) = incident_message {
            req_builder = req_builder.query(&[("incidentMessage", &s.to_string())]);
        }
        if let Some(ref s) = incident_message_like {
            req_builder = req_builder.query(&[("incidentMessageLike", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_id {
            req_builder = req_builder.query(&[("processDefinitionId", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_key_in {
            req_builder = req_builder.query(&[("processDefinitionKeyIn", &s.to_string())]);
        }
        if let Some(ref s) = process_instance_id {
            req_builder = req_builder.query(&[("processInstanceId", &s.to_string())]);
        }
        if let Some(ref s) = execution_id {
            req_builder = req_builder.query(&[("executionId", &s.to_string())]);
        }
        if let Some(ref s) = incident_timestamp_before {
            req_builder = req_builder.query(&[("incidentTimestampBefore", &s.to_string())]);
        }
        if let Some(ref s) = incident_timestamp_after {
            req_builder = req_builder.query(&[("incidentTimestampAfter", &s.to_string())]);
        }
        if let Some(ref s) = activity_id {
            req_builder = req_builder.query(&[("activityId", &s.to_string())]);
        }
        if let Some(ref s) = failed_activity_id {
            req_builder = req_builder.query(&[("failedActivityId", &s.to_string())]);
        }
        if let Some(ref s) = cause_incident_id {
            req_builder = req_builder.query(&[("causeIncidentId", &s.to_string())]);
        }
        if let Some(ref s) = root_cause_incident_id {
            req_builder = req_builder.query(&[("rootCauseIncidentId", &s.to_string())]);
        }
        //if let Some(ref s) = configuration {
        //    req_builder = req_builder.query(&[("configuration", &s.to_string())]);
        //}
        if let Some(ref s) = tenant_id_in {
            req_builder = req_builder.query(&[("tenantIdIn", &s.to_string())]);
        }
        if let Some(ref s) = job_definition_id_in {
            req_builder = req_builder.query(&[("jobDefinitionIdIn", &s.to_string())]);
        }
        if let Some(ref s) = sort_by {
            req_builder = req_builder.query(&[("sortBy", &s.to_string())]);
        }
        if let Some(ref s) = sort_order {
            req_builder = req_builder.query(&[("sortOrder", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_incidents_count(&self, incident_id: Option<&str>, incident_type: Option<&str>, incident_message: Option<&str>, incident_message_like: Option<&str>, process_definition_id: Option<&str>, process_definition_key_in: Option<&str>, process_instance_id: Option<&str>, execution_id: Option<&str>, incident_timestamp_before: Option<String>, incident_timestamp_after: Option<String>, activity_id: Option<&str>, failed_activity_id: Option<&str>, cause_incident_id: Option<&str>, root_cause_incident_id: Option<&str>, _configuration: Option<&str>, tenant_id_in: Option<&str>, job_definition_id_in: Option<&str>) -> Result<Vec<crate::models::CountResultDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/incident/count", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = incident_id {
            req_builder = req_builder.query(&[("incidentId", &s.to_string())]);
        }
        if let Some(ref s) = incident_type {
            req_builder = req_builder.query(&[("incidentType", &s.to_string())]);
        }
        if let Some(ref s) = incident_message {
            req_builder = req_builder.query(&[("incidentMessage", &s.to_string())]);
        }
        if let Some(ref s) = incident_message_like {
            req_builder = req_builder.query(&[("incidentMessageLike", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_id {
            req_builder = req_builder.query(&[("processDefinitionId", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_key_in {
            req_builder = req_builder.query(&[("processDefinitionKeyIn", &s.to_string())]);
        }
        if let Some(ref s) = process_instance_id {
            req_builder = req_builder.query(&[("processInstanceId", &s.to_string())]);
        }
        if let Some(ref s) = execution_id {
            req_builder = req_builder.query(&[("executionId", &s.to_string())]);
        }
        if let Some(ref s) = incident_timestamp_before {
            req_builder = req_builder.query(&[("incidentTimestampBefore", &s.to_string())]);
        }
        if let Some(ref s) = incident_timestamp_after {
            req_builder = req_builder.query(&[("incidentTimestampAfter", &s.to_string())]);
        }
        if let Some(ref s) = activity_id {
            req_builder = req_builder.query(&[("activityId", &s.to_string())]);
        }
        if let Some(ref s) = failed_activity_id {
            req_builder = req_builder.query(&[("failedActivityId", &s.to_string())]);
        }
        if let Some(ref s) = cause_incident_id {
            req_builder = req_builder.query(&[("causeIncidentId", &s.to_string())]);
        }
        if let Some(ref s) = root_cause_incident_id {
            req_builder = req_builder.query(&[("rootCauseIncidentId", &s.to_string())]);
        }
        //if let Some(ref s) = configuration {
        //    req_builder = req_builder.query(&[("configuration", &s.to_string())]);
        //}
        if let Some(ref s) = tenant_id_in {
            req_builder = req_builder.query(&[("tenantIdIn", &s.to_string())]);
        }
        if let Some(ref s) = job_definition_id_in {
            req_builder = req_builder.query(&[("jobDefinitionIdIn", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn resolve_incident(&self, id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/incident/{id}", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

}
