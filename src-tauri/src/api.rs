use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post, put, delete},
    Json, Router,
};
use tauri::{AppHandle, Manager};
use tower_http::cors::{Any, CorsLayer};

use crate::AppState;
use crate::customers::{
    core_create_customer, core_delete_customer, core_get_customers, core_update_customer,
    CreateCustomerPayload, Customer,
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
        .with_state(ApiState { app })
        .layer(cors)
}

pub fn spawn_server(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let port = 3000;
        let addr = format!("0.0.0.0:{}", port);
        match tokio::net::TcpListener::bind(&addr).await {
            Ok(listener) => {
                log::info!("Axum server listening on {}", addr);
                let router = build_router(app);
                if let Err(e) = axum::serve(listener, router).await {
                    log::error!("Axum server error: {}", e);
                }
            }
            Err(e) => {
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
