use std::sync::Arc;

use smart_home::{
    core::{SmartHome, SmartHomeProvider},
    database::SqliteRepository,
    provider::DeviceInfoProvider,
    webserver::{filters, RequestHandler},
};
use tokio::sync::Mutex;
use warp::Filter;

#[tokio::main]
async fn main() {
    let room_name = "Bedroom";
    let repository = SqliteRepository {};
    let mut smart_home = SmartHome::bulder("My home")
        .with_repository(repository)
        .build();
    smart_home.add_room(room_name);
    let info_provider = DeviceInfoProvider::default();

    let request_handler = Arc::new(Mutex::new(RequestHandler::new(smart_home, info_provider)));

    let api = filters::api(request_handler);
    let routes = api.with(warp::log("smarthome"));
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
