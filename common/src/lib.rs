use axum::{async_trait, extract::FromRequestParts, http::StatusCode};
use serde::Deserialize;
use tower_jwt::{DecodingKey, JwtLayer, RequestClaim, Validation};

// token : eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.RzROCXIhqwocXycXQanlthbOJRKx-yUoqIR6a93XzVs

const SECRET: &str = "toto";

#[derive(Clone, Debug, Deserialize)]
pub struct Claim {
    pub sub: String,
    pub name: String,
    pub scopes: Vec<Scope>,
}

impl Claim {
    pub fn has_scope(&self, scope: Scope) -> Result<(), StatusCode> {
        if self.scopes.contains(&scope) {
            return Ok(());
        }
        Err(StatusCode::UNAUTHORIZED)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Scope {
    Admin,
    Order,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claim {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let jwt = parts
            .extensions
            .get::<RequestClaim<Claim>>()
            .ok_or(StatusCode::UNAUTHORIZED)?;

        Ok(jwt.claim.clone())
    }
}
pub fn get_jwt_layer() -> JwtLayer<Claim, DecodingKey> {
    let mut validation = Validation::default();

    validation.required_spec_claims.clear();
    JwtLayer::<Claim>::new(validation, DecodingKey::from_secret(SECRET.as_ref()))
}
