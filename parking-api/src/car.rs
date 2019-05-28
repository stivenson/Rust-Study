#[derive(Serialize, Deserialize)]
pub struct Car {
    pub id: Option <i32>,
    pub vehicle_plate: String,
    pub years_use: i32
}
