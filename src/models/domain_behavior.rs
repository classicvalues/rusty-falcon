/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and more information about API endpoints that don't yet support OAuth2, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation). To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`. Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DomainBehavior {
    #[serde(rename = "aid", skip_serializing_if = "Option::is_none")]
    pub aid: Option<String>,
    #[serde(rename = "behavior_id", skip_serializing_if = "Option::is_none")]
    pub behavior_id: Option<String>,
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(rename = "cmdline", skip_serializing_if = "Option::is_none")]
    pub cmdline: Option<String>,
    #[serde(rename = "compound_tto", skip_serializing_if = "Option::is_none")]
    pub compound_tto: Option<String>,
    #[serde(rename = "detection_id", skip_serializing_if = "Option::is_none")]
    pub detection_id: Option<String>,
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "filepath", skip_serializing_if = "Option::is_none")]
    pub filepath: Option<String>,
    #[serde(rename = "incident_id", skip_serializing_if = "Option::is_none")]
    pub incident_id: Option<String>,
    #[serde(rename = "incident_ids", skip_serializing_if = "Option::is_none")]
    pub incident_ids: Option<Vec<String>>,
    #[serde(rename = "ioc_source", skip_serializing_if = "Option::is_none")]
    pub ioc_source: Option<String>,
    #[serde(rename = "ioc_type", skip_serializing_if = "Option::is_none")]
    pub ioc_type: Option<String>,
    #[serde(rename = "ioc_value", skip_serializing_if = "Option::is_none")]
    pub ioc_value: Option<String>,
    #[serde(rename = "objective", skip_serializing_if = "Option::is_none")]
    pub objective: Option<String>,
    #[serde(rename = "pattern_disposition", skip_serializing_if = "Option::is_none")]
    pub pattern_disposition: Option<i32>,
    #[serde(rename = "pattern_disposition_details", skip_serializing_if = "Option::is_none")]
    pub pattern_disposition_details: Option<Box<crate::models::CloudontologyPatternDisposition>>,
    #[serde(rename = "pattern_id")]
    pub pattern_id: i32,
    #[serde(rename = "sha256", skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    #[serde(rename = "tactic", skip_serializing_if = "Option::is_none")]
    pub tactic: Option<String>,
    #[serde(rename = "technique", skip_serializing_if = "Option::is_none")]
    pub technique: Option<String>,
    #[serde(rename = "template_instance_id")]
    pub template_instance_id: i32,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "user_name", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl DomainBehavior {
    pub fn new(pattern_id: i32, template_instance_id: i32, timestamp: String) -> DomainBehavior {
        DomainBehavior {
            aid: None,
            behavior_id: None,
            cid: None,
            cmdline: None,
            compound_tto: None,
            detection_id: None,
            domain: None,
            filepath: None,
            incident_id: None,
            incident_ids: None,
            ioc_source: None,
            ioc_type: None,
            ioc_value: None,
            objective: None,
            pattern_disposition: None,
            pattern_disposition_details: None,
            pattern_id,
            sha256: None,
            tactic: None,
            technique: None,
            template_instance_id,
            timestamp,
            user_name: None,
        }
    }
}
