use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::StatusCode;

use crate::webserver::models::{Device, Room};
use crate::webserver::traits::SmartHomeRepository;

//TODO Добавить логи

pub async fn list_rooms(
    repository: Arc<Mutex<impl SmartHomeRepository>>,
) -> Result<impl warp::Reply, Infallible> {
    let handler = repository.lock().await;
    let rooms = handler.get_rooms();
    Ok(warp::reply::json(&rooms))
}

pub async fn add_room(
    room: Room,
    repository: Arc<Mutex<impl SmartHomeRepository>>,
) -> Result<impl warp::Reply, Infallible> {
    let mut handler = repository.lock().await;
    match handler.add_room(room.name) {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Ok(StatusCode::BAD_REQUEST),
    }
}

pub async fn delete_room(
    room_id: u32,
    repository: Arc<Mutex<impl SmartHomeRepository>>,
) -> Result<impl warp::Reply, Infallible> {
    let mut handler = repository.lock().await;
    match handler.delete_room(room_id) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Ok(StatusCode::NOT_FOUND),
    }
}

pub async fn list_devices(
    room_id: u32,
    repository: Arc<Mutex<impl SmartHomeRepository>>,
) -> Result<impl warp::Reply, Infallible> {
    let handler = repository.lock().await;
    let devices = handler.get_devices(room_id);
    Ok(warp::reply::json(&devices))
}

pub async fn add_device(
    room_id: u32,
    device: Device,
    repository: Arc<Mutex<impl SmartHomeRepository>>,
) -> Result<impl warp::Reply, Infallible> {
    let mut handler = repository.lock().await;
    match handler.add_device(room_id, device.name) {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Ok(StatusCode::BAD_REQUEST),
    }
}

pub async fn delete_device(
    room_id: u32,
    device_id: u32,
    repository: Arc<Mutex<impl SmartHomeRepository>>,
) -> Result<impl warp::Reply, Infallible> {
    let mut handler = repository.lock().await;
    match handler.delete_device(room_id, device_id) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Ok(StatusCode::NOT_FOUND),
    }
}

pub async fn create_report(
    repository: Arc<Mutex<impl SmartHomeRepository>>,
) -> Result<impl warp::Reply, Infallible> {
    let handler = repository.lock().await;
    let report = handler.create_report();
    Ok(warp::reply::json(&report))
}
