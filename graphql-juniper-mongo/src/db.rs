use bson::{doc, oid::ObjectId};
use futures::stream::TryStreamExt;
use mongodb::{
    options::{FindOneAndReplaceOptions, ReturnDocument},
    Client, Collection,
};

use crate::allerrors::AppError;
use crate::model::Product;

pub struct Db {
    client: Client,
    db_name: String,
}

impl Db {
    pub async fn new<S>(db_name: S) -> Result<Self, AppError>
    where
        S: ToString,
    {
        let db_name = db_name.to_string();
        let client = Client::with_uri_str("mongodb://127.0.0.1:27017")
            .await
            .map_err(|e| AppError::Custom(format!("Failed to connect to MongoDB: {}", e)))?;
        Ok(Db { client, db_name })
    }

    fn get_collection(&self) -> Collection<Product> {
        self.client
            .database(&self.db_name)
            .collection::<Product>("products")
    }

    pub async fn list_products(&self) -> Result<Vec<Product>, AppError> {
        let collection = self.get_collection();
        let mut cursor = collection.find(None, None).await?;
        let mut products = Vec::new();
        
        while let Some(product) = cursor.try_next().await? {
            products.push(product);
        }
        
        Ok(products)
    }

    pub async fn get_product(&self, id: &str) -> Result<Option<Product>, AppError> {
        let collection = self.get_collection();
        let object_id = ObjectId::parse_str(id)?;
        let product = collection
            .find_one(doc! { "_id": object_id }, None)
            .await?;
        Ok(product)
    }

    pub async fn save_product(&self, mut prod: Product) -> Result<Option<Product>, AppError> {
        let collection = self.get_collection();

        if let Some(id) = prod.id {
            // Update existing product
            prod.id = None; // Remove id from the document to be updated
            let filter = doc! { "_id": id };
            let options = FindOneAndReplaceOptions::builder()
                .return_document(ReturnDocument::After)
                .build();
            
            let result = collection
                .find_one_and_replace(filter, prod, options)
                .await?;
            
            Ok(result)
        } else {
            // Insert new product
            let result = collection.insert_one(prod, None).await?;
            let inserted_id = result
                .inserted_id
                .as_object_id()
                .ok_or_else(|| AppError::Custom("No valid id returned after insert".into()))?;
            
            self.get_product(&inserted_id.to_hex()).await
        }
    }
}