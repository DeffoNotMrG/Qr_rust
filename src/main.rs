#[macro_use] extern crate rocket;

use rocket::response::content::RawXml;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use qrcode::QrCode;
use qrcode::render::svg;
use std::env;

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
    let code = QrCode::new(url).unwrap();
    let svg = code.render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build();
    
    RawXml(svg)
}

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    rocket::build().mount("/", routes![generate_qr])
}