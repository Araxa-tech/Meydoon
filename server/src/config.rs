use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub debug: bool,
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub upload_dir: String,
    pub jwt_expiry_hours: i64,
}

impl AppConfig {
    pub fn load() -> Self {
        Self {
            debug: env::var("DEBUG")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(true),
            port: env::var("PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(8000),
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:meydoon.db?mode=rwc".into()),
            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| {
                "4eea5ff117ccd6c4d8379ea9728b2840e7485c64c8091924b43c3c6745765674".into()
            }),
            upload_dir: env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".into()),
            jwt_expiry_hours: env::var("JWT_EXPIRY")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(24),
        }
    }
}
