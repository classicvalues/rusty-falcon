/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and more information about API endpoints that don't yet support OAuth2, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation). To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`. Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`batch_admin_cmd`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BatchAdminCmdError {
    Status400(crate::models::DomainApiError),
    Status403(crate::models::MsaErrorsOnly),
    Status429(crate::models::MsaReplyMetaOnly),
    Status500(crate::models::DomainApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`r_tr_check_admin_command_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RTrCheckAdminCommandStatusError {
    Status401(crate::models::DomainApiError),
    Status403(crate::models::MsaReplyMetaOnly),
    Status429(crate::models::MsaReplyMetaOnly),
    DefaultResponse(crate::models::DomainStatusResponseWrapper),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`r_tr_create_put_files`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RTrCreatePutFilesError {
    Status400(crate::models::DomainApiError),
    Status403(crate::models::MsaReplyMetaOnly),
    Status429(crate::models::MsaReplyMetaOnly),
    DefaultResponse(crate::models::MsaReplyMetaOnly),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`r_tr_create_scripts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RTrCreateScriptsError {
    Status400(crate::models::DomainApiError),
    Status403(crate::models::MsaReplyMetaOnly),
    Status429(crate::models::MsaReplyMetaOnly),
    DefaultResponse(crate::models::MsaReplyMetaOnly),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`r_tr_delete_put_files`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RTrDeletePutFilesError {
    Status400(crate::models::MsaReplyMetaOnly),
    Status403(crate::models::MsaReplyMetaOnly),
    Status404(crate::models::MsaReplyMetaOnly),
    Status429(crate::models::MsaReplyMetaOnly),
    DefaultResponse(crate::models::MsaReplyMetaOnly),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`r_tr_delete_scripts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RTrDeleteScriptsError {
    Status400(crate::models::MsaReplyMetaOnly),
    Status403(crate::models::MsaReplyMetaOnly),
    Status404(crate::models::MsaReplyMetaOnly),
    Status429(crate::models::MsaReplyMetaOnly),
    DefaultResponse(crate::models::MsaReplyMetaOnly),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`r_tr_execute_admin_command`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RTrExecuteAdminCommandError {
    Status400(crate::models::DomainApiError),
    Status403(crate::models::MsaReplyMetaOnly),
    Status429(crate::models::MsaReplyMetaOnly),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`r_tr_get_put_files`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RTrGetPutFilesError {
    Status400(crate::models::DomainApiError),
    Status403(crate::models::MsaReplyMetaOnly),
    Status404(crate::models::DomainApiError),
    Status429(crate::models::MsaReplyMetaOnly),
    DefaultResponse(crate::models::BinservclientMsaPfResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`r_tr_get_scripts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RTrGetScriptsError {
    Status400(crate::models::DomainApiError),
    Status403(crate::models::MsaReplyMetaOnly),
    Status404(crate::models::DomainApiError),
    Status429(crate::models::MsaReplyMetaOnly),
    DefaultResponse(crate::models::BinservclientMsaPfResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`r_tr_list_put_files`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RTrListPutFilesError {
    Status400(crate::models::DomainApiError),
    Status403(crate::models::MsaReplyMetaOnly),
    Status404(crate::models::DomainApiError),
    Status429(crate::models::MsaReplyMetaOnly),
    DefaultResponse(crate::models::BinservclientMsaPutFileResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`r_tr_list_scripts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RTrListScriptsError {
    Status400(crate::models::DomainApiError),
    Status403(crate::models::MsaReplyMetaOnly),
    Status404(crate::models::DomainApiError),
    Status429(crate::models::MsaReplyMetaOnly),
    DefaultResponse(crate::models::BinservclientMsaPutFileResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`r_tr_update_scripts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RTrUpdateScriptsError {
    Status400(crate::models::DomainApiError),
    Status403(crate::models::MsaReplyMetaOnly),
    Status429(crate::models::MsaReplyMetaOnly),
    DefaultResponse(crate::models::MsaReplyMetaOnly),
    UnknownValue(serde_json::Value),
}

pub async fn batch_admin_cmd(configuration: &configuration::Configuration, body: crate::models::DomainBatchExecuteCommandRequest, timeout: Option<i32>, timeout_duration: Option<&str>) -> Result<crate::models::DomainMultiCommandExecuteResponseWrapper, Error<BatchAdminCmdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/real-time-response/combined/batch-admin-command/v1", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = timeout {
        local_var_req_builder = local_var_req_builder.query(&[("timeout", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = timeout_duration {
        local_var_req_builder = local_var_req_builder.query(&[("timeout_duration", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BatchAdminCmdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn r_tr_check_admin_command_status(configuration: &configuration::Configuration, cloud_request_id: &str, sequence_id: i32) -> Result<crate::models::DomainStatusResponseWrapper, Error<RTrCheckAdminCommandStatusError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/real-time-response/entities/admin-command/v1", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("cloud_request_id", &cloud_request_id.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("sequence_id", &sequence_id.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RTrCheckAdminCommandStatusError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn r_tr_create_put_files(configuration: &configuration::Configuration, file: std::path::PathBuf, description: &str, name: Option<&str>, comments_for_audit_log: Option<&str>) -> Result<crate::models::MsaReplyMetaOnly, Error<RTrCreatePutFilesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/real-time-response/entities/put-files/v1", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form = reqwest::multipart::Form::new();
    // TODO: support file upload for 'file' parameter
    local_var_form = local_var_form.text("description", description.to_string());
    if let Some(local_var_param_value) = name {
        local_var_form = local_var_form.text("name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = comments_for_audit_log {
        local_var_form = local_var_form.text("comments_for_audit_log", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.multipart(local_var_form);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RTrCreatePutFilesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn r_tr_create_scripts(
    configuration: &configuration::Configuration,
    description: &str,
    permission_type: &str,
    file: Option<std::path::PathBuf>,
    name: Option<&str>,
    comments_for_audit_log: Option<&str>,
    content: Option<&str>,
    platform: Option<Vec<String>>,
) -> Result<crate::models::MsaReplyMetaOnly, Error<RTrCreateScriptsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/real-time-response/entities/scripts/v1", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form = reqwest::multipart::Form::new();
    // TODO: support file upload for 'file' parameter
    local_var_form = local_var_form.text("description", description.to_string());
    if let Some(local_var_param_value) = name {
        local_var_form = local_var_form.text("name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = comments_for_audit_log {
        local_var_form = local_var_form.text("comments_for_audit_log", local_var_param_value.to_string());
    }
    local_var_form = local_var_form.text("permission_type", permission_type.to_string());
    if let Some(local_var_param_value) = content {
        local_var_form = local_var_form.text("content", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = platform {
        local_var_form = local_var_form.text("platform", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
    }
    local_var_req_builder = local_var_req_builder.multipart(local_var_form);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RTrCreateScriptsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn r_tr_delete_put_files(configuration: &configuration::Configuration, ids: &str) -> Result<crate::models::MsaReplyMetaOnly, Error<RTrDeletePutFilesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/real-time-response/entities/put-files/v1", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("ids", &ids.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RTrDeletePutFilesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn r_tr_delete_scripts(configuration: &configuration::Configuration, ids: &str) -> Result<crate::models::MsaReplyMetaOnly, Error<RTrDeleteScriptsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/real-time-response/entities/scripts/v1", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("ids", &ids.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RTrDeleteScriptsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn r_tr_execute_admin_command(configuration: &configuration::Configuration, body: crate::models::DomainCommandExecuteRequest) -> Result<crate::models::DomainCommandExecuteResponseWrapper, Error<RTrExecuteAdminCommandError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/real-time-response/entities/admin-command/v1", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RTrExecuteAdminCommandError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn r_tr_get_put_files(configuration: &configuration::Configuration, ids: Vec<String>) -> Result<crate::models::BinservclientMsaPfResponse, Error<RTrGetPutFilesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/real-time-response/entities/put-files/v1", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = match "multi" {
        "multi" => local_var_req_builder.query(&ids.into_iter().map(|p| ("ids".to_owned(), p)).collect::<Vec<(std::string::String, std::string::String)>>()),
        _ => local_var_req_builder.query(&[("ids", &ids.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
    };
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RTrGetPutFilesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn r_tr_get_scripts(configuration: &configuration::Configuration, ids: Vec<String>) -> Result<crate::models::BinservclientMsaPfResponse, Error<RTrGetScriptsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/real-time-response/entities/scripts/v1", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = match "multi" {
        "multi" => local_var_req_builder.query(&ids.into_iter().map(|p| ("ids".to_owned(), p)).collect::<Vec<(std::string::String, std::string::String)>>()),
        _ => local_var_req_builder.query(&[("ids", &ids.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
    };
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RTrGetScriptsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn r_tr_list_put_files(configuration: &configuration::Configuration, filter: Option<&str>, offset: Option<&str>, limit: Option<i32>, sort: Option<&str>) -> Result<crate::models::BinservclientMsaPutFileResponse, Error<RTrListPutFilesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/real-time-response/queries/put-files/v1", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter {
        local_var_req_builder = local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RTrListPutFilesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn r_tr_list_scripts(configuration: &configuration::Configuration, filter: Option<&str>, offset: Option<&str>, limit: Option<i32>, sort: Option<&str>) -> Result<crate::models::BinservclientMsaPutFileResponse, Error<RTrListScriptsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/real-time-response/queries/scripts/v1", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter {
        local_var_req_builder = local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RTrListScriptsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn r_tr_update_scripts(
    configuration: &configuration::Configuration,
    id: &str,
    file: Option<std::path::PathBuf>,
    description: Option<&str>,
    name: Option<&str>,
    comments_for_audit_log: Option<&str>,
    permission_type: Option<&str>,
    content: Option<&str>,
    platform: Option<Vec<String>>,
) -> Result<crate::models::MsaReplyMetaOnly, Error<RTrUpdateScriptsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/real-time-response/entities/scripts/v1", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form = reqwest::multipart::Form::new();
    local_var_form = local_var_form.text("id", id.to_string());
    // TODO: support file upload for 'file' parameter
    if let Some(local_var_param_value) = description {
        local_var_form = local_var_form.text("description", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = name {
        local_var_form = local_var_form.text("name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = comments_for_audit_log {
        local_var_form = local_var_form.text("comments_for_audit_log", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = permission_type {
        local_var_form = local_var_form.text("permission_type", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = content {
        local_var_form = local_var_form.text("content", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = platform {
        local_var_form = local_var_form.text("platform", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
    }
    local_var_req_builder = local_var_req_builder.multipart(local_var_form);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RTrUpdateScriptsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
