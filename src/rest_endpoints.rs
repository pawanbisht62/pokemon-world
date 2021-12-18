use std::str;
use serde::{Deserialize, Serialize};
use log::{debug, error};

use crate::utils::PokemonError;

/// structure used for success response object
#[derive(Serialize, Deserialize)]
pub struct BasicInfo {
    name: String,
    description: String,
    habitat: String,
    isLegendary: bool,
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
                    let v: serde_json::Value = serde_json::from_str(text.as_str()).unwrap();
                    debug!("Successfully fetched details from PokeApi");
                    Ok(BasicInfo {
                        name: v["name"].as_str().unwrap().to_string(),
                        description: v["flavor_text_entries"][0]["flavor_text"]
                            .as_str()
                            .unwrap()
                            .to_string(),
                        habitat: v["habitat"]["name"].as_str().unwrap().to_string(),
                        isLegendary: v["is_legendary"].as_bool().unwrap(),
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
