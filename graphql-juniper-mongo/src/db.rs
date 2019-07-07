use bson::{from_bson, oid::ObjectId, to_bson, Bson, Document};
use mongodb::{
    coll::options::{FindOneAndUpdateOptions, ReturnDocument},
    coll::Collection,
    db::ThreadedDatabase,
    Client, ThreadedClient,
};

use crate::allerrors::Error;
use crate::model::Product;

pub struct Db {
    client: Client,
    db_name: String,
}

impl Db {
    pub fn new<S>(db_name: S) -> Db
    where
        S: ToString,
    {
        let db_name = db_name.to_string();
        let client = Client::connect("127.0.0.1", 27017).expect("Failed to initialize client.");
        Db { client, db_name }
    }

    pub fn list_products(&self) -> Result<Vec<Product>, Error> {
        let coll: Collection = self.client.db(&self.db_name).collection("products");
        let cursor = coll.find(None, None)?;
        let res: Result<Vec<_>, _> = cursor
            .map(|row| row.and_then(|item| Ok(from_bson::<Product>(Bson::Document(item))?)))
            .collect();

        Ok(res?)
    }

    pub fn get_product(&self, id: &str) -> Result<Option<Product>, Error> {
        let coll: Collection = self.client.db(&self.db_name).collection("products");
        let cursor: Option<Document> = coll.find_one(Some(doc! { "_id": ObjectId::with_string(id)? }), None)?;
        cursor
            .map(|doc| Ok(from_bson::<Product>(Bson::Document(doc))?))
            .map_or(Ok(None), |v| v.map(Some))
    }

    pub fn save_product(&self, prod: Product) -> Result<Option<Product>, Error> {
        let coll: Collection = self.client.db(&self.db_name).collection("products");

        if let Bson::Document(mut doc) = to_bson(&prod)? {
            doc.remove("_id");
            if let Some(ref id) = prod.id {
                let filter = doc!{ "_id": Bson::ObjectId(id.clone()) };
                let write_options = FindOneAndUpdateOptions {
                    return_document: Some(ReturnDocument::After),
                    ..Default::default()
                };
                let res = coll.find_one_and_replace(filter, doc, Some(write_options))?;
                if let Some(res) = res {
                    Ok(Some(from_bson::<Product>(Bson::Document(res))?))
                } else {
                    Err(Error::Custom("No data returned after update".into()))
                }
            } else {
                let res = coll.insert_one(doc, None)?;

                if let Some(exception) = res.write_exception {
                    return Err(Error::from(exception));
                }

                if let Some(inserted_id) = res.inserted_id {
                    if let Bson::ObjectId(id) = inserted_id {
                        self.get_product(&id.to_hex())
                    } else {
                        Err(Error::Custom("No valid id returned after insert".into()))
                    }
                } else {
                    Err(Error::Custom("No data returned after insert".into()))
                }
            }
        } else {
            Err(Error::Custom("Invalid document".into()))
        }
    }
}