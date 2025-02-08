use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;
use chrono::DateTime;
use thermo_sensor::{get_all_devices, get_all_readings_for_device, get_newest_readings,update_nickname};
use thermo_sensor::{parse_config, Config};

struct AppState {
    config: Config,
}

#[get("/current_reading/{date}")]
async fn newest_reading(date : web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let time_from= DateTime::parse_from_rfc3339(date.as_str()).unwrap();
    let getdata =
        get_newest_readings(data.config.db_config.clone(), time_from.into())
            .unwrap(); //todo Error handling
    web::Json(getdata)
}

#[get("/all_device_names/{date}")]
async fn all_devices(date : web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let time_from= DateTime::parse_from_rfc3339(date.as_str()).unwrap();
    let getdata =
        get_all_devices(data.config.db_config.clone(), time_from.into())
            .unwrap(); //todo error handling
    web::Json(getdata)
}

#[post("/nickname/{name}/{nickname}")]
async fn set_nickname(params : web::Path<(String,String)>, data: web::Data<AppState>) -> HttpResponse {
    let (name,nickname) = params.into_inner();
    match update_nickname(data.config.db_config.clone(),&name, &nickname){
        Ok(_)=> HttpResponse::Ok().finish(),
        Err(err)=>HttpResponse::from_error(err),
    }
}

#[get("/get_device/{name}/{date}")]
async fn device_by_name(params : web::Path<(String,String)>,  data: web::Data<AppState>) -> impl Responder {
    let (device_name,time_string) = params.into_inner();
    let time_from= DateTime::parse_from_rfc3339(&time_string).unwrap();
    let getdata =
        get_all_readings_for_device(data.config.db_config.clone(), &device_name, time_from.into())
            .unwrap(); //todo error handling
    web::Json(getdata)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let file_name = "config.toml";
    let config = parse_config(file_name).unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                config: parse_config(&file_name).unwrap()
            }))
            .service(newest_reading)
            .service(all_devices)
            .service(device_by_name)
            .service(set_nickname)
            .wrap(Cors::permissive())
    })
    .bind((config.db_config.backend_ip, config.db_config.backend_port))?
    .run()
    .await
}
