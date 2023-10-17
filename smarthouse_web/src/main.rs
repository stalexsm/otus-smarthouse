use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{web, App, HttpResponse, HttpServer, ResponseError, Scope};
use serde::{Deserialize, Serialize};

use smarthouse::devices::{Device, SmartSocket, SmartThermometer};
use smarthouse::home::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider, SmartHome};
use smarthouse::rooms::Room;

use std::error::Error as StdError;
use std::ops::Deref;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;
use uuid::Uuid;

pub type CustomResult<T> = Result<T, CustomError>;

#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum CustomError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        log::error!("Error: {}", self.to_string());
        HttpResponse::build(self.status_code()).json(self)
    }
}

#[derive(Default, Clone)]
pub struct Context {
    context: Arc<Mutex<SmartHome>>,
}

impl Context {
    pub fn new() -> Self {
        let home = SmartHome::new(String::from("Мой дом"));
        let home = Mutex::new(home);
        let home = Arc::new(home);

        Self { context: home }
    }

    pub fn get_context(&self) -> &Arc<Mutex<SmartHome>> {
        &self.context
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Data {
    name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DeviceData {
    name: String,
    device_type: DeviceType,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum DeviceType {
    Socket,
    Thermo,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Provider {
    Owning,
    Borrowing,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let ctx = Context::new();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ctx.clone()))
            .service(build_service())
            .default_service(web::to(default_response))
    })
    .bind(("127.0.0.1", 5555))?
    .run()
    .await?;

    Ok(())
}

fn build_service() -> Scope {
    web::scope("/api")
        .service(get_home)
        .service(create_room)
        .service(get_rooms)
        .service(delete_room)
        .service(create_devices)
        .service(get_devices)
        .service(delete_device)
        .service(get_reports)
}

async fn default_response() -> CustomResult<HttpResponse> {
    Ok(HttpResponse::Ok().body("Go to '/api/home'"))
}

#[actix_web::get("/home")]
async fn get_home(ctx: web::Data<Context>) -> CustomResult<HttpResponse> {
    let house = ctx.get_context().lock().await;
    let house_object = house.deref();

    Ok(HttpResponse::Ok().json(house_object))
}

#[actix_web::get("/reports/{provider}")]
async fn get_reports(
    ctx: web::Data<Context>,
    path: web::Path<Provider>,
) -> CustomResult<HttpResponse> {
    let provider = path.into_inner();

    let house = ctx.get_context().lock().await;
    match provider {
        Provider::Owning => {
            let socket = SmartSocket::new(String::from("Socket 1"));
            let info_provider_1 = OwningDeviceInfoProvider { socket };
            let report = house.create_report(&info_provider_1);

            Ok(HttpResponse::Ok().json(report))
        }
        Provider::Borrowing => {
            let socket = SmartSocket::new(String::from("Socket 1"));
            let thermo = SmartThermometer::new(String::from("Socket 1"));
            let info_provider_2 = BorrowingDeviceInfoProvider {
                socket: &socket,
                thermo: &thermo,
            };
            let report = house.create_report(&info_provider_2);

            Ok(HttpResponse::Ok().json(report))
        }
    }
}

#[actix_web::get("/rooms")]
async fn get_rooms(ctx: web::Data<Context>) -> CustomResult<HttpResponse> {
    let house = ctx.get_context().lock().await;
    let rooms: Vec<&Room> = house.get_rooms().into_iter().collect();

    Ok(HttpResponse::Ok().json(rooms))
}

#[actix_web::post("/rooms")]
async fn create_room(
    ctx: web::Data<Context>,
    body_data: web::Json<Data>,
) -> CustomResult<HttpResponse> {
    let data = body_data.into_inner();

    let mut house = ctx.get_context().lock().await;
    let room: Room = Room::new(data.name);

    house.add_room(room.clone());

    Ok(HttpResponse::Created().json(room.id))
}

#[actix_web::delete("/rooms/{id}")]
async fn delete_room(ctx: web::Data<Context>, path: web::Path<Uuid>) -> CustomResult<HttpResponse> {
    let id = path.into_inner();

    let mut house = ctx.get_context().lock().await;
    if let Some(room) = house.get_room(&id) {
        house.delete_room(room); // Удалим

        Ok(HttpResponse::Ok().json("Ok"))
    } else {
        Ok(HttpResponse::NotFound().json(CustomError::NotFound(format!("Room: {}", id))))
    }
}

#[actix_web::get("/rooms/{id}/devices")]
async fn get_devices(ctx: web::Data<Context>, path: web::Path<Uuid>) -> CustomResult<HttpResponse> {
    let id = path.into_inner();

    let house = ctx.get_context().lock().await;
    let devices = house.devices(&id);
    match devices {
        Some(devices) => Ok(HttpResponse::Ok().json(devices)),
        None => Ok(HttpResponse::NotFound().json(CustomError::NotFound(format!("Room: {}", id)))),
    }
}

#[actix_web::post("/rooms/{id}/devices")]
async fn create_devices(
    ctx: web::Data<Context>,
    path: web::Path<Uuid>,
    body_data: web::Json<DeviceData>,
) -> CustomResult<HttpResponse> {
    let id = path.into_inner();
    let data = body_data.into_inner();

    let mut house = ctx.get_context().lock().await;
    if let Some(mut room) = house.get_room(&id) {
        let device = match data.device_type {
            DeviceType::Socket => Device::Socket(SmartSocket::new(data.name)),
            DeviceType::Thermo => Device::Thermometer(SmartThermometer::new(data.name)),
        };
        room.add_device(device);
        house.add_room(room.clone());

        Ok(HttpResponse::Created().json(room))
    } else {
        Ok(HttpResponse::NotFound().json(CustomError::NotFound(format!("Room: {}", id))))
    }
}

#[actix_web::delete("/rooms/{id}/devices/{device_id}")]
async fn delete_device(
    ctx: web::Data<Context>,
    path: web::Path<(Uuid, Uuid)>,
) -> CustomResult<HttpResponse> {
    let (id, device_id) = path.into_inner();

    let mut house = ctx.get_context().lock().await;
    if let Some(mut room) = house.get_room(&id) {
        if let Some(device) = room.get_device(&device_id) {
            room.delete_device(device);
            house.add_room(room);

            Ok(HttpResponse::Ok().json("OK"))
        } else {
            Ok(HttpResponse::NotFound()
                .json(CustomError::NotFound(format!("Device: {}", device_id))))
        }
    } else {
        Ok(HttpResponse::NotFound().json(CustomError::NotFound(format!("Room: {}", id))))
    }
}
