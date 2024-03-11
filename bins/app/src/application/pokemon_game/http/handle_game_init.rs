use crate::application::pokemon_game::contract::PokemonGameContract;
use actix_web::{web, HttpRequest, HttpResponse};
use error::Error;
use support::helpers::http::{get_api_version, get_authenticated_user_id_from_request};

pub async fn handle_game_init<T: PokemonGameContract>(
    request: HttpRequest,
    service: web::Data<T>,
) -> Result<HttpResponse, Error> {
    match get_api_version(&request).as_ref() {
        "v1" => handle_game_init_v1(request, service).await,
        _ => Err(Error::NotFound),
    }
}
pub async fn handle_game_init_v1<T: PokemonGameContract>(
    request: HttpRequest,
    service: web::Data<T>,
) -> Result<HttpResponse, Error> {
    // get authenticated user id from request
    let authenticated_user_id = get_authenticated_user_id_from_request(&request)?;
    // start game, return attempt id to be shown in response
    let response = service.start_game(&authenticated_user_id).await?;
    // create response
    Ok(HttpResponse::Ok().json(response))
}
