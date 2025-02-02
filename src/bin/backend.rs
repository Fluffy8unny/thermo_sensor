use actix_web::{get, web, App, HttpServer, Responder};
use chrono::Utc;
use thermo_sensor::{get_all_devices, get_all_readings_for_device, get_newest_readings};
use thermo_sensor::{parse_config, Config};

struct AppState {
    config: Config,
}

#[get("/current_reading")]
async fn newest_reading(data: web::Data<AppState>) -> impl Responder {
    let getdata =
        get_newest_readings(data.config.db_config.clone(), Utc::now())
            .unwrap(); //todo Error handling
    web::Json(getdata)
}

#[get("/all_device_names")]
async fn all_devices(data: web::Data<AppState>) -> impl Responder {
    let getdata =
        get_all_devices(data.config.db_config.clone(), Utc::now())
            .unwrap(); //todo error handling
    web::Json(getdata)
}

#[get("/get_device/{name}")]
async fn device_by_name(name: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let device_name = name.as_str();
    let getdata =
        get_all_readings_for_device(data.config.db_config.clone(), device_name, Utc::now())
            .unwrap(); //todo error handling
    web::Json(getdata)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let file_name = "config.toml";
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                config: parse_config(file_name).unwrap(),
            }))
            .service(newest_reading)
            .service(all_devices)
            .service(device_by_name)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
