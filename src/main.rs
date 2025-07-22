use axum::{ debug_handler, Json, routing::get, Router };

#[tokio::main]
async fn main() {
    //1. build your appplication with a route
    let route01 = Router::new().route("/vehicle", get(vehicle_info).post(vehicle_post));

    //2.listen on port 3000
    let address = "0.0.0.0:5500";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    //3. run the application
    axum::serve(listener, route01).await.unwrap();
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Vehicle {
    maker: String,
    model: String,
    year: i32,
    color: String,
    price: String,
    id: Option<String>,
}
#[debug_handler]
async fn vehicle_info() -> Json<Vehicle> {
    Json::from(Vehicle {
        maker: "Nissan".to_string(),
        model: "Altima".to_string(),
        year: 2020,
        color: "Black".to_string(),
        price: "$20,000".to_string(),
        id: Some(uuid::Uuid::new_v4().to_string()),
    })
}

async fn vehicle_post() {}
