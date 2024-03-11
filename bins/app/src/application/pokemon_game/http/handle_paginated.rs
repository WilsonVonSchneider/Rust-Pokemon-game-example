use crate::application::pokemon_game::{contract::PokemonGameContract, data::RequestPaginatedData};
use actix_web::{web, HttpRequest, HttpResponse};
use error::Error;
use support::helpers::http::{
    get_api_version, get_authenticated_user_id_from_request
};

pub async fn handle_paginated<T: PokemonGameContract>(
    request: HttpRequest,
    data: web::Json<RequestPaginatedData>,
    service: web::Data<T>,
) -> Result<HttpResponse, Error> {
    match get_api_version(&request).as_ref() {
        "v1" => handle_paginated_v1(request, data, service).await,
        _ => Err(Error::NotFound),
    }
}
pub async fn handle_paginated_v1<T: PokemonGameContract>(
    request: HttpRequest,
    data: web::Json<RequestPaginatedData>,
    service: web::Data<T>,
) -> Result<HttpResponse, Error> {
    // get authenticated user id from request
    let authenticated_user_id = get_authenticated_user_id_from_request(&request)?;
    // get paginated data
    let data = data.into_inner().check();
    // return paginated list of users pokemons from pokedex
    let pokemons = service.paginated(&authenticated_user_id, data).await?;
    Ok(HttpResponse::Ok().json(pokemons))
}