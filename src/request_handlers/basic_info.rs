use std::str::FromStr;
use actix_web::{HttpResponse, web};
use log::debug;

use crate::rest_endpoints::get_pokemon_info;

static CONTENT_TYPE: &str = "application/json";
static POKEAPI_BASIC_INFO_PATH: &str = "https://pokeapi.co/api/v2/pokemon-species/";

/// This function inserts the handler for retrieving pokemon's detail
///
/// # Arguments
///
/// * `name` - name of the pokemon
///
/// # Return
///
/// This function returns the HTTPResponse will the success or error details
pub async fn get_info_handler(name: web::Path<String>) -> HttpResponse {
    match get_pokemon_info(name.as_str(), POKEAPI_BASIC_INFO_PATH).await {
        Ok(basic_info) =>  {
            debug!("Got pokemon details");
            HttpResponse::Ok()
                .content_type(CONTENT_TYPE)
                .json(basic_info)
        },
        Err(error) => {
            debug!("Error from api");
            HttpResponse::SeeOther()
                .status(
                    reqwest::StatusCode::from_str(error.error_code.as_str())
                        .unwrap_or(reqwest::StatusCode::INTERNAL_SERVER_ERROR),
                )
                .content_type(CONTENT_TYPE)
                .json(error)
        }
    }
}
