use actix_web::{App, HttpServer, web};
use log::{info};
use pokemon_world::request_handlers::basic_info::get_info_handler;
use pokemon_world::request_handlers::translated_desc::get_translated_info_handler;

static POKEMON_PATH: &str = "/pokemon";
static NAME: &str = "/{name}";
static PATH_FOR_TRANSLATED: &str = "/translated/{name}";
static SOCKET: &str = "127.0.0.1:8080";


/// This project pertains to fetch pokemon details
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Application Started...");

    HttpServer::new(|| {
        App::new().service(
            web::scope(POKEMON_PATH)
                .route(NAME, web::get().to(get_info_handler))
                .route(PATH_FOR_TRANSLATED, web::get().to(get_translated_info_handler)),
        )
    })
        .bind(SOCKET)?
        .run()
        .await
}
