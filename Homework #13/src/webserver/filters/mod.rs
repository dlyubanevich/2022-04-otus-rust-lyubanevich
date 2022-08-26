use std::sync::Arc;

use crate::core::SmartHome;
use crate::database::SqliteRepository;
use crate::provider::DeviceInfoProvider;
use crate::webserver::handlers;
use crate::webserver::models::{Device, RequestHandler, Room};
use tokio::sync::Mutex;
use warp::Filter;

pub type Repository = Arc<Mutex<RequestHandler<SmartHome<SqliteRepository>, DeviceInfoProvider>>>;

pub fn api(
    repository: Repository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    rooms_list(repository.clone())
        .or(rooms_create(repository.clone()))
        .or(rooms_delete(repository.clone()))
        .or(device_create(repository.clone()))
        .or(device_list(repository.clone()))
        .or(device_delete(repository.clone()))
        .or(report(repository))
}

fn rooms_list(
    repository: Repository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("rooms")
        .and(warp::get())
        .and(with_repository(repository))
        .and_then(handlers::list_rooms)
}
fn rooms_create(
    repository: Repository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("rooms")
        .and(warp::post())
        .and(json_room_body())
        .and(with_repository(repository))
        .and_then(handlers::add_room)
}
fn rooms_delete(
    repository: Repository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("rooms" / u32)
        .and(warp::delete())
        .and(with_repository(repository))
        .and_then(handlers::delete_room)
}
fn device_list(
    repository: Repository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("rooms" / u32)
        .and(warp::path!("devices"))
        .and(warp::get())
        .and(with_repository(repository))
        .and_then(handlers::list_devices)
}
fn device_create(
    repository: Repository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("rooms" / u32)
        .and(warp::path!("devices"))
        .and(warp::post())
        .and(json_device_body())
        .and(with_repository(repository))
        .and_then(handlers::add_device)
}
fn device_delete(
    repository: Repository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("rooms" / u32)
        .and(warp::path!("devices" / u32))
        .and(warp::delete())
        .and(with_repository(repository))
        .and_then(handlers::delete_device)
}
fn report(
    repository: Repository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("report")
        .and(warp::get())
        .and(with_repository(repository))
        .and_then(handlers::create_report)
}

fn with_repository(
    repository: Repository,
) -> impl Filter<Extract = (Repository,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || Arc::clone(&repository))
}

fn json_room_body() -> impl Filter<Extract = (Room,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
fn json_device_body() -> impl Filter<Extract = (Device,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
