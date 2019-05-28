#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::JsonValue;
use rocket_contrib::json::Json;

mod car;
use car::{Car};

#[post("/", data = "<car>")]
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

#[put("/<id>", data = "<car>")]
fn update(id: i32, car: Json<Car>) -> Json<Car> {
    car
}

#[delete("/<id>")]
fn delete(id: i32) -> JsonValue {
    json!({"status": "ok"})
}

fn main() {
    rocket::ignite()
        .mount("/car", routes![create, update, delete])
        .mount("/cars", routes![read])
        .launch();
}
