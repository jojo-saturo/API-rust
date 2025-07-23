use axum::Json;
use axum::debug_handler;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Vehicle {
    maker: String,
    model: String,
    year: i32,
    color: String,
    price: String,
    id: Option<String>,
}

#[debug_handler]
pub async fn vehicle_info() -> Json<Vehicle> {
    println!("A Retrieved Vehicle");
    Json::from(Vehicle {
        maker: "Nissan".to_string(),
        model: "Altima".to_string(),
        year: 2020,
        color: "Black".to_string(),
        price: "$20,000".to_string(),
        id: Some(uuid::Uuid::new_v4().to_string()),
    })
}
// Customers
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Customer {
    first_name: String,
    last_name: String,
}

#[debug_handler]
pub async fn customer_get() -> Json<Customer> {
    println!("A Retrieved Customer");
    Json::from(Customer {
        first_name: "Eruobami".to_string(),
        last_name: "Deborah".to_string(),
    })
}

pub async fn customer_post(Json(c): Json<Customer>) -> Json<Customer> {
    println!(
        "Customer first name: {0}, Customer Last name: {1}",
        c.first_name, c.last_name
    );
    Json::from(c)
}

pub async fn vehicle_post(Json(mut v): Json<Vehicle>) -> Json<Vehicle> {
    println!(
        "maker: {0}, model: {1}, year: {2}, color: {3}, price: {4}",
        v.maker, v.model, v.year, v.color, v.price
    );
    v.id = Some(uuid::Uuid::new_v4().to_string());
    Json::from(v)
}
