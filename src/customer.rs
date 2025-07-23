use axum::Json;
use axum::debug_handler;

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
