use rocket::serde::json::Json;
use rocket::{get, post, put, delete, routes};
use serde_json::{json, Value as JsonValue};

mod car;
use car::Car;

#[post("/", format = "json", data = "<car>")]
fn create(car: Json<Car>) -> Json<Car> {
    car
}

#[get("/")]
fn read() -> JsonValue {
    json!([
        "car 1", 
        "car 2"
    ])
}

#[put("/<id>", format = "json", data = "<car>")]
fn update(_id: i32, car: Json<Car>) -> Json<Car> {
    car
}

#[delete("/<id>")]
fn delete(_id: i32) -> JsonValue {
    json!({"status": "ok"})
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/car", routes![create, update, delete])
        .mount("/cars", routes![read])
        .launch()
        .await?;
    
    Ok(())
}
