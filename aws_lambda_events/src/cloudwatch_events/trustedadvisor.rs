use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckItemRefreshNotification {
    #[serde(rename = "check-name")]
    pub check_name: String,
    #[serde(rename = "check-item-detail")]
    pub check_item_detail: HashMap<String, String>,
    pub status: String,
    #[serde(rename = "resource_id")]
    pub resource_id: String,
    pub uuid: String,
}