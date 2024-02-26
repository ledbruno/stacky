use std::{/*env,*/ sync::{Arc, RwLock}};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use axum::{
    extract::State, http::{header::CONTENT_TYPE, HeaderValue, Method}, routing::{get, post}, Json, Router
};

#[derive(Clone, Serialize, Deserialize)]
struct Stacky {
     pub content: String,
}

#[derive(Clone)]
struct AppState {
     pub stack: Arc<RwLock<Vec<Stacky>>>,
}

impl AppState {
    pub fn new(stack: Vec<Stacky>) -> Self {
        Self{
            stack: Arc::new(RwLock::new(stack)),
        }
    }
}

pub(crate) fn load_state() -> AppState {
    AppState::new(Vec::new())
}

async fn push( State(state): State<AppState>,  Json(stacky): Json<Stacky>,) -> Json<Vec<Stacky>> {
    let mut stack = state.stack.write().expect("Error acquiring stack");
    stack.push(stacky);
    Json(stack.clone())
}

async fn pop( State(state): State<AppState>) -> Json<Stacky>  {
    let mut vec = state.stack.write().expect("Error acquiring stack");
    Json(vec.pop().expect("Error reading the stack").clone())
}

async fn show( State(state): State<AppState>) -> Json<Vec<Stacky>>  {
    let vec = state.stack.read().expect("Error acquiring stack mutex").clone();
    Json(vec.clone())
}

fn routes() -> Router {
    Router::new()
        .route("/stack/push", post(push))
        .route("/stack/pop", post(pop))
        .route("/stack/show", get(show))
        .layer(
                   CorsLayer::new()
                       .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                       .allow_headers([CONTENT_TYPE])
                       .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE]),
               )
        .with_state(load_state())
}

#[tokio::main]
async fn main() {
    //env::set_var("RUST_BACKTRACE", "full");
    let routes = routes();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes).await.unwrap();
}
use std::{/*env,*/ sync::{Arc, RwLock}};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use axum::{
    extract::State, http::{header::CONTENT_TYPE, HeaderValue, Method}, routing::{get, post}, Json, Router
};

#[derive(Clone, Serialize, Deserialize)]
struct Stacky {
     pub content: String,
}

#[derive(Clone)]
struct AppState {
     pub stack: Arc<RwLock<Vec<Stacky>>>,
}

impl AppState {
    pub fn new(stack: Vec<Stacky>) -> Self {
        Self{
            stack: Arc::new(RwLock::new(stack)),
        }
    }
}

pub(crate) fn load_state() -> AppState {
    AppState::new(Vec::new())
}

async fn push( State(state): State<AppState>,  Json(stacky): Json<Stacky>,) -> Json<Vec<Stacky>> {
    let mut stack = state.stack.write().expect("Error acquiring stack");
    stack.push(stacky);
    Json(stack.clone())
}

async fn pop( State(state): State<AppState>) -> Json<Stacky>  {
    let mut vec = state.stack.write().expect("Error acquiring stack");
    Json(vec.pop().expect("Error reading the stack").clone())
}

async fn show( State(state): State<AppState>) -> Json<Vec<Stacky>>  {
    let vec = state.stack.read().expect("Error acquiring stack mutex").clone();
    Json(vec.clone())
}

fn routes() -> Router {
    Router::new()
        .route("/stack/push", post(push))
        .route("/stack/pop", post(pop))
        .route("/stack/show", get(show))
        .layer(
                   CorsLayer::new()
                       .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                       .allow_headers([CONTENT_TYPE])
                       .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE]),
               )
        .with_state(load_state())
}

#[tokio::main]
async fn main() {
    //env::set_var("RUST_BACKTRACE", "full");
    let routes = routes();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes).await.unwrap();
}
