use std::path::PathBuf;

use actix::clock::Instant;
use actix_files::NamedFile;
use actix_web::{get, http::Cookie, web, HttpMessage, HttpRequest, HttpResponse, Responder};

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::app_state::AppStateWithCounter;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

#[derive(Serialize)]
struct Stats {
    auth_counter: u32,
    auth_average_time: f64,
    verify_counter: u32,
    verify_average_time: f64,
}

lazy_static! {
    static ref ENCODING_KEY: EncodingKey =
        match EncodingKey::from_rsa_pem(include_bytes!("../new_private.pem")) {
            Ok(key) => key,
            Err(err) => panic!("{}", err),
        };
    static ref DECODING_KEY: DecodingKey<'static> =
        match DecodingKey::from_rsa_pem(include_bytes!("../public.pem")) {
            Ok(key) => key,
            Err(err) => panic!("{}", err),
        };
    static ref VALIDATION: Validation = Validation::new(Algorithm::RS256);
}

/**
 * This is the handler function for the verify route. It expects a cookie named token
 * and then verifies and decodes the token. It returns the decoded username as the text of the body.
 */
#[get("/verify")]
async fn verify(req: HttpRequest, data: web::Data<AppStateWithCounter>) -> impl Responder {
    let mut verify_counter = data.verify_counter.lock().unwrap();
    *verify_counter += 1;

    let token_cookie = match req.cookie("token") {
        Some(token) => token,
        None => {
            return HttpResponse::BadRequest().body("No token found");
        }
    };
    let token = token_cookie.value().to_string();
    let start = Instant::now();
    let decoded_username = decode::<Claims>(&token, &DECODING_KEY, &VALIDATION);
    let duration = start.elapsed();
    let mut verify_time = data.verify_time.lock().unwrap();
    *verify_time += duration.as_millis();
    match decoded_username {
        Ok(token_data_claims) => {
            let username = token_data_claims.claims.sub;
            HttpResponse::Ok().body(username)
        }
        Err(err) => HttpResponse::BadRequest().body(format!("{}", err)),
    }
}

/**
 * The handler function for the auth route. It takes the username from path and then returns a JWT in a cookie with name token, along with the public
 * RSA key in the body.
 */
#[get("/auth/{username}")]
async fn auth_provider(req: HttpRequest, data: web::Data<AppStateWithCounter>) -> HttpResponse {
    let mut auth_counter = data.auth_counter.lock().unwrap();
    *auth_counter += 1;

    let username = req.match_info().get("username").unwrap();
    let dt = Utc::now() + Duration::hours(24);
    let claim = Claims {
        sub: username.to_string(),
        exp: dt.timestamp() as usize,
        iat: Utc::now().timestamp() as usize,
    };
    let start = Instant::now();
    let token = match encode(&Header::new(Algorithm::RS256), &claim, &ENCODING_KEY) {
        Ok(token) => token,
        Err(err) => {
            println!("{}", err);
            return HttpResponse::InternalServerError().body("Failed to encode token");
        }
    };
    let elapsed = start.elapsed();
    let mut auth_time = data.auth_time.lock().unwrap();
    *auth_time += elapsed.as_millis();
    let cookie = Cookie::build("token", token)
        .path("/")
        .secure(true)
        .http_only(true)
        .finish();
    let path: PathBuf = PathBuf::from("./public.pem");
    let file = match NamedFile::open(path) {
        Ok(file) => file,
        Err(err) => {
            println!("{}", err);
            return HttpResponse::InternalServerError().body("Failed to find public pem file");
        }
    };
    let mut res = match file.into_response(&req) {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            return HttpResponse::InternalServerError().body("Failed to covert file to response");
        }
    };
    match res.add_cookie(&cookie) {
        Ok(_) => {}
        Err(err) => {
            println!("{}", err);
            return HttpResponse::InternalServerError().body("Failed to add cookie to response");
        }
    };
    res
}

/**
 * The stats endpoint returns the current stats of the app.
 */
#[get("/stats")]
pub async fn stats(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let auth_counter = data.auth_counter.lock().unwrap();
    let auth_average_time = data.auth_time.lock().unwrap();
    let verify_counter = data.verify_counter.lock().unwrap();
    let verify_average_time = data.verify_time.lock().unwrap();
    let stats = Stats {
        auth_counter: *auth_counter,
        auth_average_time: *auth_average_time as f64 / *auth_counter as f64,
        verify_counter: *verify_counter,
        verify_average_time: *verify_average_time as f64 / *verify_counter as f64,
    };
    web::Json(stats)
}

/**
 * The readme endpoint servers the readme.txt
 */
#[get("/README.txt")]
async fn serve_readme() -> actix_web::Result<NamedFile> {
    let path: PathBuf = PathBuf::from("./README.txt");
    Ok(NamedFile::open(path)?)
}
