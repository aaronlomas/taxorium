use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post, put, delete},
    Json, Router,
};
use tauri::{AppHandle, Manager};
use tower_http::cors::{Any, CorsLayer};

use crate::emit;
use crate::AppState;
use crate::customers::{
    core_create_customer, core_delete_customer, core_get_customers, core_update_customer,
    CreateCustomerPayload, Customer,
};
use crate::sellers::{
    core_create_seller, core_delete_seller, core_get_sellers, core_login_seller,
    core_update_seller, CreateSellerPayload, LoginSellerPayload, Seller, UpdateSellerPayload,
};
use crate::products::{
    core_create_product, core_delete_product, core_get_products, core_update_product,
    CreateProductPayload, Product,
};

#[derive(Clone)]
struct ApiState {
    app: AppHandle,
}

pub fn build_router(app: AppHandle) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/customers", get(get_customers).post(create_customer))
        .route("/api/customers/:id", put(update_customer).delete(delete_customer))
        .route("/api/sellers", get(get_sellers).post(create_seller))
        .route("/api/sellers/login", post(login_seller))
        .route("/api/sellers/:id", put(update_seller).delete(delete_seller))
        .route("/api/products", get(get_products_api).post(create_product_api))
        .route("/api/products/:id", put(update_product_api).delete(delete_product_api))
        .with_state(ApiState { app })
        .layer(cors)
}

pub fn spawn_server(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let port = 3000;
        let addr = format!("0.0.0.0:{}", port);
        match tokio::net::TcpListener::bind(&addr).await {
            Ok(listener) => {
                let msg = format!("Servidor local iniciado en {}", addr);
                emit::info(&app, "api", &msg);
                log::info!("Axum server listening on {}", addr);
                let router = build_router(app.clone());
                if let Err(e) = axum::serve(listener, router).await {
                    let err_msg = format!("Error en el servidor local: {}", e);
                    emit::error(&app, "api", &err_msg);
                    log::error!("Axum server error: {}", e);
                }
            }
            Err(e) => {
                let err_msg = format!("No se pudo vincular el servidor local en {}: {}", addr, e);
                emit::error(&app, "api", &err_msg);
                log::error!("Failed to bind Axum server to {}: {}", addr, e);
            }
        }
    });
}

// Helper to run blocking DB operations
async fn run_db_task<F, R>(state: State<ApiState>, task: F) -> Result<R, (StatusCode, String)>
where
    F: FnOnce(&rusqlite::Connection) -> Result<R, String> + Send + 'static,
    R: Send + 'static,
{
    let app = state.app.clone();
    
    let result = tokio::task::spawn_blocking(move || {
        let app_state = app.state::<AppState>();
        let db_guard = app_state.db.lock().map_err(|e| e.to_string())?;
        let db = db_guard.as_ref().ok_or("Database not initialized")?;
        task(db)
    })
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    result.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}

async fn get_customers(state: State<ApiState>) -> Result<Json<Vec<Customer>>, (StatusCode, String)> {
    let customers = run_db_task(state, |db| core_get_customers(db)).await?;
    Ok(Json(customers))
}

async fn create_customer(
    state: State<ApiState>,
    Json(payload): Json<CreateCustomerPayload>,
) -> Result<Json<Customer>, (StatusCode, String)> {
    let customer = run_db_task(state, move |db| core_create_customer(db, payload)).await?;
    Ok(Json(customer))
}

async fn update_customer(
    state: State<ApiState>,
    Path(id): Path<i64>,
    Json(payload): Json<CreateCustomerPayload>,
) -> Result<Json<Customer>, (StatusCode, String)> {
    let customer = run_db_task(state, move |db| core_update_customer(db, id, payload)).await?;
    Ok(Json(customer))
}

async fn delete_customer(
    state: State<ApiState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    run_db_task(state, move |db| core_delete_customer(db, id)).await?;
    Ok(StatusCode::NO_CONTENT)
}

// --- Sellers handlers ---

async fn get_sellers(state: State<ApiState>) -> Result<Json<Vec<Seller>>, (StatusCode, String)> {
    let sellers = run_db_task(state, |db| core_get_sellers(db)).await?;
    Ok(Json(sellers))
}

async fn create_seller(
    state: State<ApiState>,
    Json(payload): Json<CreateSellerPayload>,
) -> Result<Json<Seller>, (StatusCode, String)> {
    let device_id = machine_uid::get().unwrap_or_else(|_| "default_secure_password_123!".to_string());
    let seller = run_db_task(state, move |db| core_create_seller(db, payload, &device_id)).await?;
    Ok(Json(seller))
}

async fn update_seller(
    state: State<ApiState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateSellerPayload>,
) -> Result<Json<Seller>, (StatusCode, String)> {
    let device_id = machine_uid::get().unwrap_or_else(|_| "default_secure_password_123!".to_string());
    let seller = run_db_task(state, move |db| core_update_seller(db, id, payload, &device_id)).await?;
    Ok(Json(seller))
}

async fn delete_seller(
    state: State<ApiState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    run_db_task(state, move |db| core_delete_seller(db, id)).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn login_seller(
    state: State<ApiState>,
    Json(payload): Json<LoginSellerPayload>,
) -> Result<Json<Seller>, (StatusCode, String)> {
    let device_id = machine_uid::get().unwrap_or_else(|_| "default_secure_password_123!".to_string());
    let seller = run_db_task(state, move |db| core_login_seller(db, payload, &device_id)).await?;
    Ok(Json(seller))
}

// --- Products handlers ---

async fn get_products_api(state: State<ApiState>) -> Result<Json<Vec<Product>>, (StatusCode, String)> {
    let products = run_db_task(state, |db| core_get_products(db)).await?;
    Ok(Json(products))
}

async fn create_product_api(
    state: State<ApiState>,
    Json(payload): Json<CreateProductPayload>,
) -> Result<Json<Product>, (StatusCode, String)> {
    let product = run_db_task(state, move |db| core_create_product(db, payload)).await?;
    Ok(Json(product))
}

async fn update_product_api(
    state: State<ApiState>,
    Path(id): Path<i64>,
    Json(payload): Json<CreateProductPayload>,
) -> Result<Json<Product>, (StatusCode, String)> {
    let product = run_db_task(state, move |db| core_update_product(db, id, payload)).await?;
    Ok(Json(product))
}

async fn delete_product_api(
    state: State<ApiState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    run_db_task(state, move |db| core_delete_product(db, id)).await?;
    Ok(StatusCode::NO_CONTENT)
}

