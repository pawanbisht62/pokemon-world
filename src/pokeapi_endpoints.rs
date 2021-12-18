use std::collections::HashMap;
use std::str;
use serde::{Deserialize, Serialize};
use log::{debug, error};

use crate::utils::PokemonError;

/// structure used for success response object
#[derive(Serialize, Deserialize)]
pub struct BasicInfo {
    pub(crate) name: String,
    pub description: String,
    pub habitat: String,
    pub isLegendary: bool,
}

/// This function fetches the pokemon detail from the PokeAPI
///
/// # Arguments
///
/// * `name` - name of the pokemon
///
/// * `api_path` - path of the pokeApi's endpoint
///
/// # Return
///
/// This function returns the error object of the api
pub async fn get_pokemon_info(
    name: &str,
    api_path: &str,
) -> Result<BasicInfo, PokemonError> {
    match reqwest::get(api_path.to_owned() + name).await {
        Ok(response) => match response.status() {
            reqwest::StatusCode::OK => match response.text().await {
                Ok(text) => {
                    let json_body: serde_json::Value = serde_json::from_str(text.as_str()).unwrap();
                    debug!("Successfully fetched details from PokeApi");
                    Ok(BasicInfo {
                        name: json_body["name"].as_str().unwrap().to_string(),
                        description: json_body["flavor_text_entries"][0]["flavor_text"]
                            .as_str()
                            .unwrap()
                            .to_string(),
                        habitat: json_body["habitat"]["name"].as_str().unwrap().to_string(),
                        isLegendary: json_body["is_legendary"].as_bool().unwrap(),
                    })
                }
                Err(resp_decode_error) => {
                    error!("error while fetching response text:{}", resp_decode_error);
                    Err(PokemonError::get_error_detail(
                        reqwest::StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                        Some(resp_decode_error.to_string()),
                    ))
                }
            },
            _ => {
                debug!("error status code:{}", response.status().to_string());
                Err(PokemonError::get_error_detail(
                    response.status().to_string(),
                    None,
                ))
            }
        },
        Err(endpoint_error) => {
            error!("error in api call:{}", endpoint_error);
            Err(PokemonError::get_error_detail(
                reqwest::StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                Some(endpoint_error.to_string()),
            ))
        }
    }
}




/// This function fetches the translated text from the PokeAPI
///
/// # Arguments
///
/// * `api_path` - path of the pokeApi's endpoint
///
/// * `body` - request body
///
/// # Return
///
/// This function returns the error object of the api
pub async fn get_translated_text(
    api_path: &str,
    body: HashMap<&str, &str>
) -> Result<String, PokemonError> {
    match reqwest::Client::new().post(api_path)
        .json(&body)
        .send()
        .await
    {
        Ok(response) => match response.status() {
            reqwest::StatusCode::OK => match response.text().await {
                Ok(text) => {
                    let json_body: serde_json::Value = serde_json::from_str(text.as_str()).unwrap();
                    debug!("Successfully fetched translated text from Yoda");
                    Ok(json_body["contents"]["translated"]
                            .as_str()
                            .unwrap()
                            .to_string()
                    )
                }
                Err(resp_decode_error) => {
                    error!("error while fetching response body:{}", resp_decode_error);
                    Err(PokemonError::get_error_detail(
                        reqwest::StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                        Some(resp_decode_error.to_string()),
                    ))
                }
            },
            _ => {
                debug!("error status code:{}", response.status().to_string());
                Err(PokemonError::get_error_detail(
                    response.status().to_string(),
                    None,
                ))
            }
        },
        Err(endpoint_error) => {
            error!("error in yoda api call:{}", endpoint_error);
            Err(PokemonError::get_error_detail(
                reqwest::StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                Some(endpoint_error.to_string()),
            ))
        }
    }
}
