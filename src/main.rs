#[macro_use] extern crate rocket;

use rocket::response::content::RawXml;
use rocket::http::{Status, Header};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::config::Config;
use rocket::{Response};
use rocket::fairing::{Fairing, Info, Kind};
use qrcode::QrCode;
use qrcode::render::svg;
use std::env;
use std::net::IpAddr;
use log::{info, error};

#[cfg(test)]
mod tests;

struct ApiKey<'r>(&'r str);

#[derive(Debug)]
enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let api_key = req.headers().get_one("X-API-Key");
        match api_key {
            None => Outcome::Error((Status::BadRequest, ApiKeyError::Missing)),
            Some(key) => {
                let correct_api_key = env::var("API_KEY").expect("API_KEY must be set");
                if key == correct_api_key {
                    Outcome::Success(ApiKey(key))
                } else {
                    Outcome::Error((Status::Unauthorized, ApiKeyError::Invalid))
                }
            }
        }
    }
}

#[get("/generate?<url>")]
fn generate_qr(_api_key: ApiKey<'_>, url: String) -> RawXml<String> {
    info!("Generating QR code for URL: {}", url);
    let code = QrCode::new(url).unwrap();
    let svg = code.render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build();
    
    RawXml(svg)
}

#[get("/")]
fn hello() -> &'static str {
    info!("Hello route accessed");
    "Hello, world!"
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "GET, POST, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type, X-API-Key"));
    }
}

#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

#[launch]
fn rocket() -> _ {
    if std::env::var("RUST_LOG").is_err() {
        env_logger::init();
    }
    
    dotenv::dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port: u16 = port.parse().expect("PORT must be a number");

    info!("Using port: {}", port);

    let config = Config {
        port,
        address: IpAddr::from([0, 0, 0, 0]),
        ..Config::default()
    };

    info!("Rocket config: {:?}", config);

    rocket::custom(config)
        .mount("/", routes![generate_qr, hello, all_options])
        .attach(CORS)
}