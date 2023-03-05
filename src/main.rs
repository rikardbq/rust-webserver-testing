use std::{path::PathBuf, str};
use env_logger::Env;
use log::{debug, info};
use actix_files::NamedFile;
use actix_web::{get, Result, App, HttpResponse, HttpRequest, HttpServer, Responder};
use serde_json::Value as SerdeValue;

mod helpers;
use helpers::curl_helper;

const API_KEY: &str = "3015d4f03f43caefb8b947ea96c1656f";
const LAT: f32 = 57.7;
const LNG: f32 = 12.0;


#[get("/")]
async fn index(_req: HttpRequest) -> Result<NamedFile> {
    let f_path: PathBuf = "./static/index.html".parse().unwrap();
    debug!("{}", f_path.to_str().unwrap());

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
    let weather_api_path = format!("https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}", LAT, LNG, API_KEY);
    let res = curl_helper::get_from(&weather_api_path);
    let v: SerdeValue = serde_json::from_str(res.as_str()).unwrap();
    info!("Main {}\nSys {}\nWeather {}", v["main"]["humidity"], v["sys"], v["weather"][0]["main"]);

    let weather = &v["weather"][0]["main"].to_string();
    let humidity = &v["main"]["humidity"].to_string();
    let pressure = &v["main"]["pressure"].to_string();
    let temp = &v["main"]["temp"].to_string();
    let sunrise = &v["sys"]["sunrise"].to_string();
    let sunset = &v["sys"]["sunset"].to_string();
    let div = r#"
    <div id="weather">
        <div id="1">
            <h3>Weather</h3>
            <p>{weather}</p>
            <ul>
                <li>humidity: {humidity}</li>
                <li>pressure: {pressure}</li>
                <li>temp: {temp}</li>
                <li>sunrise: {sunrise}</li>
                <li>sunset: {sunset}</li>
            </ul>
        </div>
    </div>
    "#;

    let result_div = div
        .replace("{weather}", weather)
        .replace("{humidity}", humidity)
        .replace("{pressure}", pressure)
        .replace("{temp}", temp)
        .replace("{sunrise}", sunrise)
        .replace("{sunset}", sunset);

    HttpResponse::Ok().body(result_div)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let log_level = "info";
    env_logger::init_from_env(Env::default().default_filter_or(log_level));
    
    HttpServer::new(|| {
        App::new()
        .service(home)
        .service(weather)
        .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}