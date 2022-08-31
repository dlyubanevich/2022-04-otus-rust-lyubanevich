use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::StatusCode;

use crate::webserver::models::{Device, RequestHandler, Room};

pub async fn list_rooms(
    repository: Arc<Mutex<RequestHandler>>,
) -> Result<impl warp::Reply, Infallible> {
    let handler = repository.lock().await;
    let rooms = handler.get_rooms().await;
    Ok(warp::reply::json(&rooms))
}

pub async fn add_room(
    room: Room,
    repository: Arc<Mutex<RequestHandler>>,
) -> Result<impl warp::Reply, Infallible> {
    let mut handler = repository.lock().await;
    match handler.add_room(room.name).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Ok(StatusCode::BAD_REQUEST),
    }
}

pub async fn delete_room(
    room_name: String,
    repository: Arc<Mutex<RequestHandler>>,
) -> Result<impl warp::Reply, Infallible> {
    let mut handler = repository.lock().await;
    match handler.delete_room(room_name).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Ok(StatusCode::NOT_FOUND),
    }
}

pub async fn list_devices(
    room_name: String,
    repository: Arc<Mutex<RequestHandler>>,
) -> Result<impl warp::Reply, Infallible> {
    let handler = repository.lock().await;
    let devices = handler.get_devices(room_name).await;
    Ok(warp::reply::json(&devices))
}

pub async fn add_device(
    room_name: String,
    device: Device,
    repository: Arc<Mutex<RequestHandler>>,
) -> Result<impl warp::Reply, Infallible> {
    let mut handler = repository.lock().await;
    match handler.add_device(room_name, device.name).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Ok(StatusCode::BAD_REQUEST),
    }
}

pub async fn delete_device(
    room_name: String,
    device_name: String,
    repository: Arc<Mutex<RequestHandler>>,
) -> Result<impl warp::Reply, Infallible> {
    let mut handler = repository.lock().await;
    match handler.delete_device(room_name, device_name).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Ok(StatusCode::NOT_FOUND),
    }
}

pub async fn create_report(
    repository: Arc<Mutex<RequestHandler>>,
) -> Result<impl warp::Reply, Infallible> {
    let handler = repository.lock().await;
    let report = handler.create_report().await;
    Ok(warp::reply::json(&report))
}
