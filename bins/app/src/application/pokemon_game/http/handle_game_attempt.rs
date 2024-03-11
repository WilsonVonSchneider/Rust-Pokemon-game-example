use crate::application::pokemon_game::{contract::PokemonGameContract, data::RequestAttemptData};
use actix_web::{web, HttpRequest, HttpResponse};
use error::Error;
use support::helpers::http::{
    get_api_version, get_authenticated_user_id_from_request, part_from_path,
};
use validr::Validation;

pub async fn handle_game_attempt<T: PokemonGameContract>(
    request: HttpRequest,
    data: web::Json<RequestAttemptData>,
    service: web::Data<T>,
) -> Result<HttpResponse, Error> {
    match get_api_version(&request).as_ref() {
        "v1" => handle_game_attempt_v1(request, data, service).await,
        _ => Err(Error::NotFound),
    }
}
pub async fn handle_game_attempt_v1<T: PokemonGameContract>(
    request: HttpRequest,
    data: web::Json<RequestAttemptData>,
    service: web::Data<T>,
) -> Result<HttpResponse, Error> {
    // get authenticated user id from request
    let authenticated_user_id = get_authenticated_user_id_from_request(&request)?;
    // get guess from data
    let guess = data.into_inner().validate()?.guess.unwrap();
    // get attempt id from url
    let attempt_id = part_from_path::<String>(&request, "attempt_id")?;
    // check guess
    service
        .check_guess(&authenticated_user_id, &guess, &attempt_id)
        .await?;
    // create response
    Ok(HttpResponse::Ok().json("Success. Pokemon saved into your pokedex!!!"))
}
