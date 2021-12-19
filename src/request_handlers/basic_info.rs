use actix_web::{web, HttpResponse};
use log::debug;
use std::str::FromStr;

use crate::pokeapi_endpoints::get_pokemon_info;

pub static CONTENT_TYPE: &str = "application/json";
pub static POKEAPI_BASIC_INFO_PATH: &str = "https://pokeapi.co/api/v2/pokemon-species/";

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
        Ok(basic_info) => {
            debug!("Got pokemon details");
            HttpResponse::Ok()
                .content_type(CONTENT_TYPE)
                .json(basic_info)
        }
        Err(error) => {
            debug!("Error from basic_info handler {:?}", &error);
            HttpResponse::SeeOther()
                .status(
                    reqwest::StatusCode::from_str(&error.error_code.as_str()[..3])
                        .unwrap_or(reqwest::StatusCode::INTERNAL_SERVER_ERROR),
                )
                .content_type(CONTENT_TYPE)
                .json(error)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::request_handlers::basic_info::get_info_handler;
    use actix_web::web::Path;

    #[actix_web::test]
    async fn test_get_info_handler_success() {
        let res = get_info_handler(Path::from("mewtwo".to_string())).await;
        assert!(res.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_info_handler_failure() {
        let res = get_info_handler(Path::from("mewtwo1".to_string())).await;
        assert!(res.status().is_client_error());
    }
}
