use super::domain::Register;
use super::infrastructure::{PgService, PgRepository};
use actix_web::web;
use super::http::*;
use infrastructure::db::Pg;
use std::sync::Arc;


pub fn routes(pg_pool: Arc<Pg>, cfg: &mut web::ServiceConfig) {
    let service = Register {
        service: PgService { pg_pool: pg_pool.clone() },
        repository: PgRepository {pg_pool}
    };
    cfg.app_data(web::Data::new(service));
    cfg.route(
        "/auth/register",
        web::post().to(handle_create::<Register<PgRepository, PgService>>)
    );
    cfg.route(
        "/auth/verify-email/resend",
        web::post().to(handle_verify_email_resend::<Register<PgRepository, PgService>>)
    );
}
