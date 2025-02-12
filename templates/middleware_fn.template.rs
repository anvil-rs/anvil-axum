use axum::{
    Router,
    http::StatusCode,
    routing::get,
    response::{IntoResponse, Response},
    middleware::{self, Next},
    extract::{Request, Extension},
};

async fn {{name|snakecase}}(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    unimplemented!()
}
