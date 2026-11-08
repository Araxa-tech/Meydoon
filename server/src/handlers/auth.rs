use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::{Duration, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set, TransactionTrait,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{
    auth::{jwt, password as pw, token},
    entity::{password_reset_token, profile, revoke_token, user},
    error::AppError,
    middleware::AuthUser,
    state::AppState,
};

const PASSWORD_RESET_TTL_HOURS: i64 = 1;

// ---------- payloads ----------

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RegisterRequest {
    pub email: String,
    pub user_name: String,
    pub phone_number: String,
    pub password: String,
    #[serde(default)]
    pub first_name: String,
    #[serde(default)]
    pub last_name: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub identifier: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}

// ---------- response shapes ----------

#[derive(Serialize)]
pub struct UserWithProfile {
    #[serde(flatten)]
    pub user: user::Model,
    pub profile: Option<profile::Model>,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub token_type: &'static str,
    pub expires_in: i64,
    pub user: UserWithProfile,
}

// ---------- validation helpers ----------

fn valid_email(e: &str) -> bool {
    if e.is_empty() || e.len() > 80 {
        return false;
    }
    let mut it = e.split('@');
    match (it.next(), it.next(), it.next()) {
        (Some(l), Some(d), None) => !l.is_empty() && !d.is_empty() && d.contains('.'),
        _ => false,
    }
}
fn valid_user_name(s: &str) -> bool {
    (3..=80).contains(&s.len())
}
fn valid_phone(s: &str) -> bool {
    (5..=15).contains(&s.len())
}
fn valid_password(s: &str) -> bool {
    (8..=72).contains(&s.len())
} // bcrypt 72-byte cap

fn issue_token(state: &AppState, uid: Uuid) -> Result<(String, i64), AppError> {
    let hours = if state.config.jwt_expiry_hours > 0 {
        state.config.jwt_expiry_hours
    } else {
        24
    };
    let token = jwt::generate(&uid.to_string(), &state.config.jwt_secret, hours)
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
    Ok((token, hours * 3600))
}

// ---------- handlers ----------

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    if !valid_email(&req.email) {
        return Err(AppError::Validation("email: must be a valid email".into()));
    }
    if !valid_user_name(&req.user_name) {
        return Err(AppError::Validation("user_name: must be 3-80 chars".into()));
    }
    if !valid_phone(&req.phone_number) {
        return Err(AppError::Validation(
            "phone_number: must be 5-15 chars".into(),
        ));
    }
    if !valid_password(&req.password) {
        return Err(AppError::Validation("password: must be 8-72 chars".into()));
    }

    let email = req.email.trim().to_lowercase();
    let user_name = req.user_name.trim().to_string();
    let phone = req.phone_number.trim().to_string();

    let existing = user::Entity::find()
        .filter(
            Condition::any()
                .add(user::Column::Email.eq(&email))
                .add(user::Column::UserName.eq(&user_name))
                .add(user::Column::PhoneNumber.eq(&phone)),
        )
        .one(&state.db)
        .await?;

    if let Some(u) = existing {
        let field = if u.email == email {
            "email"
        } else if u.user_name == user_name {
            "user_name"
        } else {
            "phone_number"
        };
        return Err(AppError::Conflict(format!("{field} already in use")));
    }

    let hash = pw::hash_password(&req.password)
        .map_err(|_| AppError::BadRequest("invalid password".into()))?;

    let txn = state.db.begin().await?;
    let now = Utc::now();
    let user_id = Uuid::new_v4();

    let created_user = user::ActiveModel {
        id: Set(user_id),
        email: Set(email),
        user_name: Set(user_name),
        phone_number: Set(phone),
        password: Set(hash),
        first_name: Set(req.first_name.trim().to_string()),
        last_name: Set(req.last_name.trim().to_string()),
        is_active: Set(true),
        is_staff: Set(false),
        is_superuser: Set(false),
        is_verified: Set(false),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    }
    .insert(&txn)
    .await?;

    let created_profile = profile::ActiveModel {
        user_id: Set(user_id),
        profile_pic: Set("profile_pics/default.jpg".into()),
        profile_banner: Set("profile_banners/default.jpg".into()),
        bio: Set(String::new()),
        ..Default::default()
    }
    .insert(&txn)
    .await?;

    txn.commit().await?;

    let (token, expires_in) = issue_token(&state, created_user.id)?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            token,
            token_type: "Bearer",
            expires_in,
            user: UserWithProfile {
                user: created_user,
                profile: Some(created_profile),
            },
        }),
    ))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let ident = req.identifier.trim().to_string();
    let lower = ident.to_lowercase();

    let user = user::Entity::find()
        .filter(
            Condition::any()
                .add(user::Column::Email.eq(&lower))
                .add(user::Column::UserName.eq(&ident)),
        )
        .one(&state.db)
        .await?;

    let Some(user) = user else {
        return Err(AppError::Unauthorized("invalid credentials".into()));
    };
    if !pw::check_password(&user.password, &req.password) {
        return Err(AppError::Unauthorized("invalid credentials".into()));
    }
    if !user.is_active {
        return Err(AppError::Forbidden("account is disabled".into()));
    }

    let profile = profile::Entity::find()
        .filter(profile::Column::UserId.eq(user.id))
        .one(&state.db)
        .await?;

    let (token, expires_in) = issue_token(&state, user.id)?;

    Ok(Json(AuthResponse {
        token,
        token_type: "Bearer",
        expires_in,
        user: UserWithProfile { user, profile },
    }))
}

pub async fn logout(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let expires_at = chrono::DateTime::from_timestamp(auth.exp, 0)
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("bad exp in token")))?;

    revoke_token::Entity::delete_many()
        .filter(revoke_token::Column::ExpiresAt.lt(Utc::now()))
        .exec(&state.db)
        .await?;

    revoke_token::Entity::insert(revoke_token::ActiveModel {
        jti: Set(auth.jti),
        expires_at: Set(expires_at),
    })
    .on_conflict(
        sea_orm::sea_query::OnConflict::column(revoke_token::Column::Jti)
            .do_nothing()
            .to_owned(),
    )
    .exec(&state.db)
    .await?;

    Ok(Json(json!({ "message": "logged out" })))
}

pub async fn me(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let row = user::Entity::find_by_id(auth.id)
        .find_also_related(profile::Entity)
        .one(&state.db)
        .await?;
    let Some((user, profile)) = row else {
        return Err(AppError::Unauthorized("user no longer exists".into()));
    };
    if !user.is_active {
        return Err(AppError::Forbidden("account is disabled".into()));
    }
    Ok(Json(json!({
        "user": UserWithProfile { user, profile },
    })))
}

pub async fn forgot_password(
    State(state): State<AppState>,
    Json(req): Json<ForgotPasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    if !valid_email(&req.email) {
        return Err(AppError::Validation("email: must be a valid email".into()));
    }
    let email = req.email.trim().to_lowercase();

    let generic = json!({ "message": "if the account exists, a reset token has been sent" });

    let Some(user) = user::Entity::find()
        .filter(user::Column::Email.eq(&email))
        .one(&state.db)
        .await?
    else {
        return Ok(Json(generic));
    };

    let raw = token::generate_random(32); // 256 bits
    let hash = token::hash_token(&raw);
    let expires_at = Utc::now() + Duration::hours(PASSWORD_RESET_TTL_HOURS);

    let txn = state.db.begin().await?;
    password_reset_token::Entity::delete_many()
        .filter(password_reset_token::Column::UserId.eq(user.id))
        .filter(password_reset_token::Column::UsedAt.is_null())
        .exec(&txn)
        .await?;
    password_reset_token::ActiveModel {
        user_id: Set(user.id),
        token_hash: Set(hash),
        expires_at: Set(expires_at),
        created_at: Set(Utc::now()),
        ..Default::default()
    }
    .insert(&txn)
    .await?;
    txn.commit().await?;

    // In production: deliver raw via email/SMS. Debug behaviour matches the Go API.
    let mut resp = generic;
    if state.config.debug {
        resp["reset_token"] = json!(raw);
        resp["expires_at"] = json!(expires_at);
    }
    Ok(Json(resp))
}

pub async fn reset_password(
    State(state): State<AppState>,
    Json(req): Json<ResetPasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    if !valid_password(&req.new_password) {
        return Err(AppError::Validation(
            "new_password: must be 8-72 chars".into(),
        ));
    }

    let hash = token::hash_token(req.token.trim());

    let reset = password_reset_token::Entity::find()
        .filter(password_reset_token::Column::TokenHash.eq(&hash))
        .one(&state.db)
        .await?;
    let Some(reset) = reset else {
        return Err(AppError::BadRequest("invalid or expired token".into()));
    };
    if reset.used_at.is_some() || Utc::now() > reset.expires_at {
        return Err(AppError::BadRequest("invalid or expired token".into()));
    }

    let new_hash = pw::hash_password(&req.new_password)
        .map_err(|_| AppError::BadRequest("invalid password".into()))?;

    let txn = state.db.begin().await?;
    user::ActiveModel {
        id: Set(reset.user_id),
        password: Set(new_hash),
        updated_at: Set(Utc::now()),
        ..Default::default()
    }
    .update(&txn)
    .await?;
    password_reset_token::ActiveModel {
        id: Set(reset.id),
        used_at: Set(Some(Utc::now())),
        ..Default::default()
    }
    .update(&txn)
    .await?;
    password_reset_token::Entity::delete_many()
        .filter(password_reset_token::Column::UserId.eq(reset.user_id))
        .filter(password_reset_token::Column::Id.ne(reset.id))
        .filter(password_reset_token::Column::UsedAt.is_null())
        .exec(&txn)
        .await?;
    txn.commit().await?;

    Ok(Json(json!({ "message": "password has been reset" })))
}
