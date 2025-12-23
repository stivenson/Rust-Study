use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Custom error: {0}")]
    Custom(String),

    #[error("MongoDB error: {0}")]
    Mongo(#[from] mongodb::error::Error),

    #[error("BSON serialization error: {0}")]
    BsonSer(#[from] bson::ser::Error),

    #[error("BSON deserialization error: {0}")]
    BsonDe(#[from] bson::de::Error),

    #[error("Invalid ObjectId: {0}")]
    BsonOid(#[from] bson::oid::Error),
}