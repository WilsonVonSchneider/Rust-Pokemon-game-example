use actix_web::{web, HttpResponse};
use infrastructure::db::Pg;
use std::sync::Arc;

pub fn configure(pg_pool: Arc<Pg>, cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/_health",
        web::get().to(|| async { HttpResponse::Ok().body("I'm healthy".to_string()) }),
    );
    //Auth routes
    auth(pg_pool.clone(), cfg);
    //Pokemon game routes
    pokemon_game(pg_pool, cfg);
}
// auth routes
fn auth(p: Arc<Pg>, c: &mut web::ServiceConfig) {
    crate::application::auth::registration::setup::routes(p.clone(), c);
    crate::application::auth::login::setup::routes(p, c);
}
// pokemon game routes
fn pokemon_game(p: Arc<Pg>, c: &mut web::ServiceConfig) {
    crate::application::pokemon_game::setup::routes(p, c);
}
