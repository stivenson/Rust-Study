use std::sync::Arc;
use warp::Filter;

mod allerrors;
mod db;
mod gql;
mod model;

use db::Db;
use gql::{Mutations, Query};

#[derive(Clone)]
pub struct Context {
    pub db: Arc<Db>,
}

impl juniper::Context for Context {}

pub type Schema = juniper::RootNode<'static, Query, Mutations>;

fn schema() -> Schema {
    Schema::new(Query, Mutations)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Initialize database connection
    let db = Db::new("mydb-to-graphql-juniper")
        .await
        .expect("Failed to connect to MongoDB");
    
    let ctx = Context { db: Arc::new(db) };
    let schema = schema();

    // GraphiQL UI endpoint
    let graphiql_route = warp::path::end()
        .and(warp::get())
        .map(|| {
            warp::reply::html(juniper::http::graphiql::graphiql_source("/graphql", None))
        });

    // GraphQL endpoint
    let graphql_route = juniper_warp::make_graphql_filter(schema, warp::any().map(move || ctx.clone()).boxed());

    let routes = graphiql_route
        .or(warp::path("graphql").and(graphql_route));

    println!("GraphQL server running at http://127.0.0.1:3030");
    println!("GraphiQL UI available at http://127.0.0.1:3030");
    
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
