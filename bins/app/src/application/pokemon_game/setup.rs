use super::domain::PokemonGame;
use super::http::*;
use super::infrastructure::{PgRepository, PgService};
use actix_web::web;
use infrastructure::db::Pg;
use std::sync::Arc;

pub fn routes(pg_pool: Arc<Pg>, cfg: &mut web::ServiceConfig) {
    let service = PokemonGame {
        service: PgService {
            pg_pool: pg_pool.clone(),
        },
        repository: PgRepository { pg_pool },
    };
    cfg.app_data(web::Data::new(service));
    cfg.route(
        "/api/{version}/pokemon-game",
        web::get().to(handle_game_init::<PokemonGame<PgRepository, PgService>>),
    );
    cfg.route(
        "/api/{version}/pokemon-game/attempt/{attempt_id}",
        web::post().to(handle_game_attempt::<PokemonGame<PgRepository, PgService>>),
    );
    cfg.route(
        "/api/{version}/users/pokedex",
        web::get().to(handle_paginated::<PokemonGame<PgRepository, PgService>>),
    );
}
