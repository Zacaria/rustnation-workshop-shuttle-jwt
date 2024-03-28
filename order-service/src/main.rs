use axum::{http::StatusCode, routing::get, Router};
use common::Claim;

async fn handler(claim: Claim) -> Result<String, StatusCode> {
    claim.has_scope(common::Scope::Order)?;

    Ok(format!(
        "Hello, {} ({}) from the order service!",
        claim.name, claim.sub
    ))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(handler))
        .layer(common::get_jwt_layer());

    Ok(router.into())
}
