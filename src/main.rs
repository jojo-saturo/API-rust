use axum::{Router, routing::get};

mod vehicle;

mod customer;

use vehicle::{vehicle_info, vehicle_post};

use customer::{customer_get, customer_post};

#[tokio::main]
async fn main() {
    //1. build your appplication with a route
    let route01 = Router::new()
        .route("/vehicle", get(vehicle_info).post(vehicle_post))
        .route("/customer", get(customer_get).post(customer_post));

    //2.listen on port 3000
    let address = "0.0.0.0:5500";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    //3. run the application
    axum::serve(listener, route01).await.unwrap();
}
