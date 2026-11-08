use axum::{
    Router,
    http::StatusCode,
    routing::{delete, get, post, put},
};
use tower_http::trace::TraceLayer;

use crate::{handlers::{
    auth as ah, gig, gig_bid as gb, gig_comment as gc, gig_comment_reply as gcr,
    gig_image as gi, gig_rating as gr, order as od,
}, state::AppState};

pub fn build(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { StatusCode::OK }))
        // Auth
        .route("/api/auth/register", post(ah::register))
        .route("/api/auth/login", post(ah::login))
        .route("/api/auth/logout", post(ah::logout))
        .route("/api/auth/forgot-password", post(ah::forgot_password))
        .route("/api/auth/reset-password", post(ah::reset_password))
        .route("/api/auth/me", get(ah::me))

        // gigs
        .route("/api/gigs", get(gig::list).post(gig::create))
        .route("/api/gigs/:id", get(gig::get).put(gig::update).delete(gig::delete))
        // images
        .route("/api/gigs/:id/images", post(gi::add))
        .route("/api/gigs/:gig_id/images/:image_id", delete(gi::remove))
        // rating
        .route("/api/gigs/:id/rating", post(gr::upsert).delete(gr::delete))
        // comments
        .route("/api/gigs/:id/comments", get(gc::list).post(gc::create))
        .route("/api/gigs/:gig_id/comments/:cid", put(gc::update).delete(gc::delete))
        // replies
        .route(
            "/api/gigs/:gig_id/comments/:cid/replies",
            get(gcr::list).post(gcr::create),
        )
        .route(
            "/api/gigs/:gig_id/comments/:cid/replies/:rid",
            put(gcr::update).delete(gcr::delete),
        )
        // bids
        .route("/api/gigs/:id/bids", get(gb::list).post(gb::create))
        .route("/api/gigs/:gig_id/bids/:bid_id", put(gb::update).delete(gb::withdraw))
        .route("/api/gigs/:gig_id/bids/:bid_id/accept", post(gb::accept))
        .route("/api/gigs/:gig_id/bids/:bid_id/reject", post(gb::reject))
        // orders
        .route("/api/orders", get(od::list))
        .route("/api/orders/:id", get(od::get))
        .route("/api/orders/:id/cancel", post(od::cancel))
        .route("/api/orders/:id/deliver", post(od::deliver))
        .route("/api/orders/:id/complete", post(od::complete))
        .route("/api/orders/:id/dispute", post(od::dispute))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
