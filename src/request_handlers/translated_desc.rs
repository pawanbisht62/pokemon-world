use std::collections::HashMap;
use std::str::FromStr;

use actix_web::{HttpResponse, web};
use log::{debug};

use crate::request_handlers::basic_info::{CONTENT_TYPE, POKEAPI_BASIC_INFO_PATH};
use crate::pokeapi_endpoints::{BasicInfo, get_translated_text, get_pokemon_info};

static POKEAPI_TRANSLATED_YODA_PATH: &str = "https://api.funtranslations.com/translate/yoda.json";
static POKEAPI_TRANSLATED_SHAKESPEARE_PATH: &str = "https://api.funtranslations.com/translate/shakespeare.json";

/// This function inserts the handler for retrieving pokemon's detail
///
/// # Arguments
///
/// * `name` - name of the pokemon
///
/// # Return
///
/// This function returns the HTTPResponse will the success or error details
pub async fn get_translated_info_handler(name: web::Path<String>) -> HttpResponse {
    match get_pokemon_info(name.as_str(), POKEAPI_BASIC_INFO_PATH).await {
        Ok(basic_info) => {
            let mut json_body: HashMap<&str, &str> = HashMap::new();
            json_body.insert("text", basic_info.description.as_str());

            if basic_info.habitat == "Cave" || basic_info.isLegendary{
                match get_translated_text(POKEAPI_TRANSLATED_YODA_PATH, json_body).await {
                    Ok(translated_text) => {
                        let info = BasicInfo {
                            name: basic_info.name,
                            description: translated_text,
                            habitat: basic_info.habitat,
                            isLegendary: basic_info.isLegendary,
                        };
                        HttpResponse::Ok()
                            .content_type(CONTENT_TYPE)
                            .json(info)
                    }
                    Err(yoda_error) => {
                        debug!("Error from yoda api");
                        HttpResponse::SeeOther()
                            .status(
                                reqwest::StatusCode::from_str(yoda_error.error_code.as_str())
                                    .unwrap_or(reqwest::StatusCode::INTERNAL_SERVER_ERROR),
                            )
                            .content_type(CONTENT_TYPE)
                            .json(yoda_error)
                    }
                }
            } else {
                match get_translated_text(POKEAPI_TRANSLATED_SHAKESPEARE_PATH, json_body).await {
                    Ok(translated_text) => {
                        let info = BasicInfo {
                            name: basic_info.name,
                            description: translated_text,
                            habitat: basic_info.habitat,
                            isLegendary: basic_info.isLegendary,
                        };
                        HttpResponse::Ok()
                            .content_type(CONTENT_TYPE)
                            .json(info)
                    }
                    Err(shakespeare_error) => {
                        debug!("Error from shakespeare api");
                        HttpResponse::SeeOther()
                            .status(
                                reqwest::StatusCode::from_str(shakespeare_error.error_code.as_str())
                                    .unwrap_or(reqwest::StatusCode::INTERNAL_SERVER_ERROR),
                            )
                            .content_type(CONTENT_TYPE)
                            .json(shakespeare_error)
                    }
                }

            }
        }
        Err(error) => {
            debug!("Error from basic_info api");
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

