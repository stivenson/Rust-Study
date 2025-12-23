use bson::oid::ObjectId;
use juniper::{graphql_object, FieldResult, GraphQLObject};

use super::Context;
use crate::model::Product as ProductModel;

#[derive(GraphQLObject, Clone)]
#[graphql(context = Context)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub tp: i32,
    pub qty: i32,
    pub price: i32,
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub weight: i32,
    pub description: String,
}

impl From<ProductModel> for Product {
    fn from(p: ProductModel) -> Self {
        Product {
            id: p.id.map(|id| id.to_hex()).unwrap_or_default(),
            name: p.name,
            slug: p.slug,
            tp: p.tp,
            qty: p.qty,
            price: p.price,
            width: p.width,
            height: p.height,
            depth: p.depth,
            weight: p.weight,
            description: p.description,
        }
    }
}

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn api_version() -> &'static str {
        "2.0"
    }

    async fn products(context: &Context) -> FieldResult<Vec<Product>> {
        let products = context.db.list_products().await?;
        Ok(products.into_iter().map(Product::from).collect())
    }

    async fn product(context: &Context, id: String) -> FieldResult<Option<Product>> {
        let product = context.db.get_product(&id).await?;
        Ok(product.map(Product::from))
    }
}

pub struct Mutations;

#[graphql_object(context = Context)]
impl Mutations {
    async fn save_product(
        context: &Context,
        id: Option<String>,
        name: String,
        slug: Option<String>,
        tp: i32,
        qty: i32,
        price: i32,
        width: Option<i32>,
        height: Option<i32>,
        depth: Option<i32>,
        weight: Option<i32>,
        description: Option<String>,
    ) -> FieldResult<Option<Product>> {
        let object_id = match id {
            Some(id_str) => Some(ObjectId::parse_str(&id_str)?),
            None => None,
        };

        let product = ProductModel {
            id: object_id,
            name,
            slug: slug.unwrap_or_default(),
            tp,
            qty,
            price,
            width: width.unwrap_or(0),
            height: height.unwrap_or(0),
            depth: depth.unwrap_or(0),
            weight: weight.unwrap_or(0),
            description: description.unwrap_or_default(),
        };

        let result = context.db.save_product(product).await?;
        Ok(result.map(Product::from))
    }
}