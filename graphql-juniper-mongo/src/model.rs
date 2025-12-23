use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub slug: String,

    #[serde(default)]
    pub tp: i32,

    #[serde(default)]
    pub qty: i32,

    #[serde(default)]
    pub price: i32,

    #[serde(default)]
    pub width: i32,

    #[serde(default)]
    pub height: i32,

    #[serde(default)]
    pub depth: i32,

    #[serde(default)]
    pub weight: i32,

    #[serde(default)]
    pub description: String,
}
