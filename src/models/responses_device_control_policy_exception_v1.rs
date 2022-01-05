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
pub struct ResponsesDeviceControlPolicyExceptionV1 {
    #[serde(rename = "action")]
    pub action: String,
    /// USB Class ID to apply the exception. If empty it applies to all classes
    #[serde(rename = "class")]
    pub class: String,
    #[serde(rename = "combined_id")]
    pub combined_id: String,
    /// Unique identifier for an exception
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "match_method")]
    pub match_method: String,
    #[serde(rename = "product_id")]
    pub product_id: String,
    #[serde(rename = "product_name")]
    pub product_name: String,
    #[serde(rename = "serial_number")]
    pub serial_number: String,
    /// Hexadecimal VendorID used to apply the exception
    #[serde(rename = "vendor_id")]
    pub vendor_id: String,
    /// Vendor Name
    #[serde(rename = "vendor_name")]
    pub vendor_name: String,
}

impl ResponsesDeviceControlPolicyExceptionV1 {
    pub fn new(action: String, class: String, combined_id: String, id: String, match_method: String, product_id: String, product_name: String, serial_number: String, vendor_id: String, vendor_name: String) -> ResponsesDeviceControlPolicyExceptionV1 {
        ResponsesDeviceControlPolicyExceptionV1 {
            action,
            class,
            combined_id,
            id,
            match_method,
            product_id,
            product_name,
            serial_number,
            vendor_id,
            vendor_name,
        }
    }
}
