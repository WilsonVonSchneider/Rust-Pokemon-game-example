use super::domain::Login;
use super::infrastructure::{PgService, PgRepository};
use actix_web::web;
use super::http::*;
use infrastructure::db::Pg;
use std::sync::Arc;


pub fn routes(pg_pool: Arc<Pg>, cfg: &mut web::ServiceConfig) {
    let service = Login {
        service: PgService { pg_pool: pg_pool.clone() },
        repository: PgRepository {pg_pool}
    };
    cfg.app_data(web::Data::new(service));
    cfg.route(
        "/auth/login",
        web::post().to(handle_login::<Login<PgRepository, PgService>>)
    );
    cfg.route(
        "/auth/verify-email/{action_token}",
        web::post().to(handle_verify_email::<Login<PgRepository, PgService>>)
    );
    cfg.route(
        "/auth/logout",
        web::post().to(handle_logout::<Login<PgRepository, PgService>>)
    );
    cfg.route(
        "/auth/refresh",
        web::get().to(handle_refresh::<Login<PgRepository, PgService>>)
    );
}