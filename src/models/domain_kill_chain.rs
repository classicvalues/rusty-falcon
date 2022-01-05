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
pub struct DomainKillChain {
    #[serde(rename = "actions_and_objectives", skip_serializing_if = "Option::is_none")]
    pub actions_and_objectives: Option<String>,
    #[serde(rename = "command_and_control", skip_serializing_if = "Option::is_none")]
    pub command_and_control: Option<String>,
    #[serde(rename = "delivery", skip_serializing_if = "Option::is_none")]
    pub delivery: Option<String>,
    #[serde(rename = "exploitation", skip_serializing_if = "Option::is_none")]
    pub exploitation: Option<String>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<String>,
    #[serde(rename = "objectives", skip_serializing_if = "Option::is_none")]
    pub objectives: Option<String>,
    #[serde(rename = "reconnaissance", skip_serializing_if = "Option::is_none")]
    pub reconnaissance: Option<String>,
    #[serde(rename = "rich_text_actions_and_objectives", skip_serializing_if = "Option::is_none")]
    pub rich_text_actions_and_objectives: Option<String>,
    #[serde(rename = "rich_text_command_and_control", skip_serializing_if = "Option::is_none")]
    pub rich_text_command_and_control: Option<String>,
    #[serde(rename = "rich_text_delivery", skip_serializing_if = "Option::is_none")]
    pub rich_text_delivery: Option<String>,
    #[serde(rename = "rich_text_exploitation", skip_serializing_if = "Option::is_none")]
    pub rich_text_exploitation: Option<String>,
    #[serde(rename = "rich_text_installation", skip_serializing_if = "Option::is_none")]
    pub rich_text_installation: Option<String>,
    #[serde(rename = "rich_text_objectives", skip_serializing_if = "Option::is_none")]
    pub rich_text_objectives: Option<String>,
    #[serde(rename = "rich_text_reconnaissance", skip_serializing_if = "Option::is_none")]
    pub rich_text_reconnaissance: Option<String>,
    #[serde(rename = "rich_text_weaponization", skip_serializing_if = "Option::is_none")]
    pub rich_text_weaponization: Option<String>,
    #[serde(rename = "weaponization", skip_serializing_if = "Option::is_none")]
    pub weaponization: Option<String>,
}

impl DomainKillChain {
    pub fn new() -> DomainKillChain {
        DomainKillChain {
            actions_and_objectives: None,
            command_and_control: None,
            delivery: None,
            exploitation: None,
            installation: None,
            objectives: None,
            reconnaissance: None,
            rich_text_actions_and_objectives: None,
            rich_text_command_and_control: None,
            rich_text_delivery: None,
            rich_text_exploitation: None,
            rich_text_installation: None,
            rich_text_objectives: None,
            rich_text_reconnaissance: None,
            rich_text_weaponization: None,
            weaponization: None,
        }
    }
}
