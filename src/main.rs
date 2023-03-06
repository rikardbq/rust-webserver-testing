use std::{path::PathBuf, str};
use env_logger::Env;
use actix_files::NamedFile;
use actix_web::{get, Result, App, HttpResponse, HttpRequest, HttpServer, Responder};
use serde_json::{Value as SerdeValue};
use serde::Serialize;

mod tera_func;
use tera_func::*;

mod helpers;
use helpers::{curl_helper, misc_helper::k_to_c};

const API_KEY: &str = "3015d4f03f43caefb8b947ea96c1656f";
const LAT: f32 = 57.7;
const LNG: f32 = 12.0;

#[derive(Serialize)]
struct IndexPage {
    wbox_html: String
}

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {

    let mut esc_vader = String::new();
    html_escape::decode_html_entities_to_string(&mk_fragment("weather", &_weather_proxy()), &mut esc_vader);
    
    let ipage = IndexPage { wbox_html: esc_vader };
    let value = serde_json::to_value(ipage).unwrap();
    let index = mk_page("front", &value);

    HttpResponse::Ok().body(index)
}

#[get("/stil")]
async fn styles(_req: HttpRequest) -> Result<NamedFile> {
    let f_path: PathBuf = "./static/css/actix-test.css".parse().unwrap();
    Ok(NamedFile::open(f_path)?)
}

#[get("/home")]
async fn home(req: HttpRequest) -> impl Responder {
    let connection_info = req.connection_info();
    let remote_ip = connection_info.realip_remote_addr().unwrap();
    
    HttpResponse::Ok().body(remote_ip.to_string())
}

#[get("/weather")]
async fn weather(_req: HttpRequest) -> impl Responder {
    let vader_div = mk_fragment("weather", &_weather_proxy());
    HttpResponse::Ok().body(vader_div)
}

fn _weather_proxy() -> SerdeValue {
    let weather_api_path = format!("https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}", LAT, LNG, API_KEY);
    let res = curl_helper::get_from(&weather_api_path);
    let mut vader_val:SerdeValue = serde_json::from_str(res.as_str()).unwrap();

    // OpenWeather har förövrigt en parameter "mode" kan anges
    // med "metric" så man får temperatur i Celsius.

    // .. men det här är roligare ;-D
    // och demonstrerar den litet snålt dokumenterade metoden `pointer_mut`.
    // man kan också manipulera Tera::Context ifall man behöver ändra inputen till mallarna.

    let deg_c:f64 = k_to_c(vader_val["main"]["temp"].as_f64().unwrap());
    *vader_val.pointer_mut("/main/temp").unwrap() = format!("{:1.1} °C", deg_c).into();

    vader_val
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let log_level = "info";
    env_logger::init_from_env(Env::default().default_filter_or(log_level));
    
    HttpServer::new(|| {
        App::new()
        .service(home)
        .service(styles)
        .service(weather)
        .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}