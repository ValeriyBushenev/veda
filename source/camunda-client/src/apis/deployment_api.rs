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

pub struct DeploymentApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl DeploymentApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> DeploymentApiClient {
        DeploymentApiClient {
            configuration,
        }
    }
}

pub trait DeploymentApi {
    fn create_deployment(&self, tenant_id: Option<&str>, deployment_source: Option<&str>, deploy_changed_only: Option<bool>, enable_duplicate_filtering: Option<bool>, deployment_name: Option<&str>, data: Option<std::path::PathBuf>) -> Result<crate::models::DeploymentWithDefinitionsDto, Error>;
    fn delete_deployment(&self, id: &str, cascade: Option<bool>, skip_custom_listeners: Option<bool>, skip_io_mappings: Option<bool>) -> Result<(), Error>;
    fn get_deployment(&self, id: &str) -> Result<Vec<crate::models::DeploymentDto>, Error>;
    fn get_deployment_resource(&self, id: &str, resource_id: &str) -> Result<crate::models::DeploymentResourceDto, Error>;
    fn get_deployment_resource_data(&self, id: &str, resource_id: &str) -> Result<std::path::PathBuf, Error>;
    fn get_deployment_resources(&self, id: &str) -> Result<Vec<crate::models::DeploymentResourceDto>, Error>;
    fn get_deployments(&self, id: Option<&str>, name: Option<&str>, name_like: Option<&str>, source: Option<&str>, without_source: Option<bool>, tenant_id_in: Option<&str>, without_tenant_id: Option<bool>, include_deployments_without_tenant_id: Option<bool>, after: Option<String>, before: Option<String>, sort_by: Option<&str>, sort_order: Option<&str>, first_result: Option<i32>, max_results: Option<i32>) -> Result<Vec<crate::models::DeploymentDto>, Error>;
    fn get_deployments_count(&self, id: Option<&str>, name: Option<&str>, name_like: Option<&str>, source: Option<&str>, without_source: Option<bool>, tenant_id_in: Option<&str>, without_tenant_id: Option<bool>, include_deployments_without_tenant_id: Option<bool>, after: Option<String>, before: Option<String>) -> Result<crate::models::CountResultDto, Error>;
    fn redeploy(&self, id: &str, redeployment_dto: Option<crate::models::RedeploymentDto>) -> Result<crate::models::DeploymentWithDefinitionsDto, Error>;
}

impl DeploymentApi for DeploymentApiClient {
    fn create_deployment(&self, tenant_id: Option<&str>, deployment_source: Option<&str>, deploy_changed_only: Option<bool>, enable_duplicate_filtering: Option<bool>, deployment_name: Option<&str>, data: Option<std::path::PathBuf>) -> Result<crate::models::DeploymentWithDefinitionsDto, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/deployment/create", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        let mut form = reqwest::multipart::Form::new();
        if let Some(param_value) = tenant_id {
            form = form.text("tenant-id", param_value.to_string());
        }
        if let Some(param_value) = deployment_source {
            form = form.text("deployment-source", param_value.to_string());
        }
        if let Some(param_value) = deploy_changed_only {
            form = form.text("deploy-changed-only", param_value.to_string());
        }
        if let Some(param_value) = enable_duplicate_filtering {
            form = form.text("enable-duplicate-filtering", param_value.to_string());
        }
        if let Some(param_value) = deployment_name {
            form = form.text("deployment-name", param_value.to_string());
        }
        if let Some(param_value) = data {
            form = form.file("data", param_value)?;
        }
        req_builder = req_builder.multipart(form);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn delete_deployment(&self, id: &str, cascade: Option<bool>, skip_custom_listeners: Option<bool>, skip_io_mappings: Option<bool>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/deployment/{id}", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref s) = cascade {
            req_builder = req_builder.query(&[("cascade", &s.to_string())]);
        }
        if let Some(ref s) = skip_custom_listeners {
            req_builder = req_builder.query(&[("skipCustomListeners", &s.to_string())]);
        }
        if let Some(ref s) = skip_io_mappings {
            req_builder = req_builder.query(&[("skipIoMappings", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn get_deployment(&self, id: &str) -> Result<Vec<crate::models::DeploymentDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/deployment/{id}", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_deployment_resource(&self, id: &str, resource_id: &str) -> Result<crate::models::DeploymentResourceDto, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/deployment/{id}/resources/{resourceId}", configuration.base_path, id=crate::apis::urlencode(id), resourceId=crate::apis::urlencode(resource_id));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_deployment_resource_data(&self, id: &str, resource_id: &str) -> Result<std::path::PathBuf, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/deployment/{id}/resources/{resourceId}/data", configuration.base_path, id=crate::apis::urlencode(id), resourceId=crate::apis::urlencode(resource_id));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_deployment_resources(&self, id: &str) -> Result<Vec<crate::models::DeploymentResourceDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/deployment/{id}/resources", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_deployments(&self, id: Option<&str>, name: Option<&str>, name_like: Option<&str>, source: Option<&str>, without_source: Option<bool>, tenant_id_in: Option<&str>, without_tenant_id: Option<bool>, include_deployments_without_tenant_id: Option<bool>, after: Option<String>, before: Option<String>, sort_by: Option<&str>, sort_order: Option<&str>, first_result: Option<i32>, max_results: Option<i32>) -> Result<Vec<crate::models::DeploymentDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/deployment", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = id {
            req_builder = req_builder.query(&[("id", &s.to_string())]);
        }
        if let Some(ref s) = name {
            req_builder = req_builder.query(&[("name", &s.to_string())]);
        }
        if let Some(ref s) = name_like {
            req_builder = req_builder.query(&[("nameLike", &s.to_string())]);
        }
        if let Some(ref s) = source {
            req_builder = req_builder.query(&[("source", &s.to_string())]);
        }
        if let Some(ref s) = without_source {
            req_builder = req_builder.query(&[("withoutSource", &s.to_string())]);
        }
        if let Some(ref s) = tenant_id_in {
            req_builder = req_builder.query(&[("tenantIdIn", &s.to_string())]);
        }
        if let Some(ref s) = without_tenant_id {
            req_builder = req_builder.query(&[("withoutTenantId", &s.to_string())]);
        }
        if let Some(ref s) = include_deployments_without_tenant_id {
            req_builder = req_builder.query(&[("includeDeploymentsWithoutTenantId", &s.to_string())]);
        }
        if let Some(ref s) = after {
            req_builder = req_builder.query(&[("after", &s.to_string())]);
        }
        if let Some(ref s) = before {
            req_builder = req_builder.query(&[("before", &s.to_string())]);
        }
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
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_deployments_count(&self, id: Option<&str>, name: Option<&str>, name_like: Option<&str>, source: Option<&str>, without_source: Option<bool>, tenant_id_in: Option<&str>, without_tenant_id: Option<bool>, include_deployments_without_tenant_id: Option<bool>, after: Option<String>, before: Option<String>) -> Result<crate::models::CountResultDto, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/deployment/count", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = id {
            req_builder = req_builder.query(&[("id", &s.to_string())]);
        }
        if let Some(ref s) = name {
            req_builder = req_builder.query(&[("name", &s.to_string())]);
        }
        if let Some(ref s) = name_like {
            req_builder = req_builder.query(&[("nameLike", &s.to_string())]);
        }
        if let Some(ref s) = source {
            req_builder = req_builder.query(&[("source", &s.to_string())]);
        }
        if let Some(ref s) = without_source {
            req_builder = req_builder.query(&[("withoutSource", &s.to_string())]);
        }
        if let Some(ref s) = tenant_id_in {
            req_builder = req_builder.query(&[("tenantIdIn", &s.to_string())]);
        }
        if let Some(ref s) = without_tenant_id {
            req_builder = req_builder.query(&[("withoutTenantId", &s.to_string())]);
        }
        if let Some(ref s) = include_deployments_without_tenant_id {
            req_builder = req_builder.query(&[("includeDeploymentsWithoutTenantId", &s.to_string())]);
        }
        if let Some(ref s) = after {
            req_builder = req_builder.query(&[("after", &s.to_string())]);
        }
        if let Some(ref s) = before {
            req_builder = req_builder.query(&[("before", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn redeploy(&self, id: &str, redeployment_dto: Option<crate::models::RedeploymentDto>) -> Result<crate::models::DeploymentWithDefinitionsDto, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/deployment/{id}/redeploy", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&redeployment_dto);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

}