use bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    #[serde(rename = "_id")]
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
