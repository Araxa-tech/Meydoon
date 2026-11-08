use std::convert::Infallible;

use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts},
};
use sea_orm::EntityTrait;
use uuid::Uuid;

use crate::{auth::jwt, entity::revoke_token, error::AppError, state::AppState};

pub struct OptionalAuthUser(pub Option<AuthUser>);

#[async_trait]
impl FromRequestParts<AppState> for OptionalAuthUser {
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self(AuthUser::from_request_parts(parts, state).await.ok()))
    }
}

pub struct AuthUser {
    pub id: Uuid,
    pub jti: Uuid,
    pub exp: i64,
}

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| {
                AppError::Unauthorized("missing or malformed authorization header".into())
            })?;

        let (scheme, token) = header.split_once(' ').ok_or_else(|| {
            AppError::Unauthorized("missing or malformed authorization header".into())
        })?;
        if !scheme.eq_ignore_ascii_case("Bearer") {
            return Err(AppError::Unauthorized(
                "missing or malformed authorization header".into(),
            ));
        }
        let token = token.trim();
        if token.is_empty() {
            return Err(AppError::Unauthorized(
                "missing or malformed authorization header".into(),
            ));
        }

        let claims = jwt::parse(token, &state.config.jwt_secret)
            .map_err(|_| AppError::Unauthorized("invalid or expired token".into()))?;

        if revoke_token::Entity::find_by_id(claims.jti)
            .one(&state.db).await?.is_some() {
            return Err(AppError::Unauthorized("token has been revoked".into()));
        }

        let id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Unauthorized("invalid token subject".into()))?;

        Ok(AuthUser { id, jti: claims.jti, exp: claims.exp })
    }
}
