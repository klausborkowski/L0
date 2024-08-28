use axum::{
    extract::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tracing::info;
use tracing_subscriber;

#[derive(Serialize, Deserialize, Clone)]
struct Delivery {
    name: String,
    phone: String,
    zip: String,
    city: String,
    address: String,
    region: String,
    email: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Payment {
    transaction: String,
    request_id: String,
    currency: String,
    provider: String,
    amount: i32,
    payment_dt: i64,
    bank: String,
    delivery_cost: i32,
    goods_total: i32,
    custom_fee: i32,
}

#[derive(Serialize, Deserialize, Clone)]
struct Item {
    chrt_id: i32,
    track_number: String,
    price: i32,
    rid: String,
    name: String,
    sale: i32,
    size: String,
    total_price: i32,
    nm_id: i32,
    brand: String,
    status: i32,
}

#[derive(Serialize, Deserialize, Clone)]
struct Order {
    order_uid: String,
    track_number: String,
    entry: String,
    delivery: Delivery,
    payment: Payment,
    items: Vec<Item>,
    locale: String,
    internal_signature: String,
    customer_id: String,
    delivery_service: String,
    shardkey: String,
    sm_id: i32,
    date_created: String,
    oof_shard: String,
}

#[derive(Serialize, Deserialize)]
struct OrderResponse {
    order_uid: String,
    status: String,
}

type SharedState = Arc<Mutex<Vec<Order>>>;

async fn get_orders(state: Arc<Mutex<Vec<Order>>>) -> Json<Vec<Order>> {
    info!("Handling GET /orders request");
    let orders = state.lock().unwrap(); 
    Json(orders.clone()) 
}

async fn create_order(
    state: Arc<Mutex<Vec<Order>>>,
    Json(order): Json<Order>,
) -> Json<OrderResponse> {
    info!("Handling POST /order request");
    let mut orders = state.lock().unwrap(); 
    orders.push(order.clone());

    Json(OrderResponse {
        order_uid: order.order_uid,
        status: "pending".to_string(),
    })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); 

    let shared_state: SharedState = Arc::new(Mutex::new(Vec::new())); 

    let app = Router::new()
        .route(
            "/orders",
            get({
                let state = shared_state.clone();
                move || get_orders(state) 
            }),
        )
        .route(
            "/order",
            post({
                let state = shared_state.clone();
                move |json| create_order(state, json) 
            }),
        );

    let addr = "127.0.0.1:8080".parse().unwrap();
    info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
