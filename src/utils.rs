use log::debug;
use serde::Serialize;

static ERROR_404: &str = "Unable to find the details of requested pokemon";
static ERROR_503: &str = "Service Unavailable";
static ERROR_500: &str = "Internal Server Error";
static HTTP_STATUS_CODE_500: &str = "500";

/// struct used to return error object
#[derive(Serialize, Debug)]
pub struct PokemonError {
    pub error_code: String,
    error_detail: String,
}

impl PokemonError {
    /// This function prepares error object
    ///
    /// # Arguments
    ///
    /// * `status_code` - http_status_code of the error
    ///
    /// * `detail` - detail of the error
    ///
    /// # Return
    ///
    /// This function returns the error object of the api
    pub fn get_error_detail(status_code: String, detail: Option<String>) -> PokemonError {
        debug!("Error occurred");
        match status_code.as_str() {
            "404 Not Found" => PokemonError {
                error_code: status_code,
                error_detail: detail.unwrap_or_else(|| ERROR_404.to_string()),
            },
            "503 Service Unavailable" => PokemonError {
                error_code: status_code,
                error_detail: detail.unwrap_or_else(|| ERROR_503.to_string()),
            },
            _ => PokemonError {
                error_code: HTTP_STATUS_CODE_500.to_string(),
                error_detail: detail.unwrap_or_else(|| ERROR_500.to_string()),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::utils::PokemonError;

    #[test]
    fn test_get_error_detail_404() {
        let error = PokemonError::get_error_detail("404 Not Found".to_string(), None);
        assert_eq!(
            error.error_detail,
            "Unable to find the details of requested pokemon".to_string()
        );
    }

    #[test]
    fn test_get_error_detail_503() {
        let error = PokemonError::get_error_detail("503 Service Unavailable".to_string(), None);
        assert_eq!(error.error_detail, "Service Unavailable".to_string());
    }

    #[test]
    fn test_get_error_detail_500() {
        let error = PokemonError::get_error_detail("500".to_string(), None);
        assert_eq!(error.error_detail, "Internal Server Error".to_string());
    }
}
