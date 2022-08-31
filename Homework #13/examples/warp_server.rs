use dotenv::dotenv;
use std::sync::Arc;

use smart_home::{
    core::{SmartHome, SmartHomeService},
    database::SqliteRepository,
    devices::Socket,
    provider::{DeviceInfoProvider, DeviceType},
    webserver::{filters, RequestHandler},
};
use tokio::sync::Mutex;

/// Example of request:
/// POST http://127.0.0.1:3030/rooms with json body { "name": "Bedroom" } - add new room
/// GET http://127.0.0.1:3030/rooms - get list of rooms
/// POST http://127.0.0.1:3030/rooms/Bedroom with json body { "name": "Socket" } - add new device
/// GET http://127.0.0.1:3030/rooms/Bedroom - get list of devices
/// GET http://127.0.0.1:3030/report - get report of devices
/// DELETE http://127.0.0.1:3030/rooms/Bedroom/Socket - delete device
/// DELETE http://127.0.0.1:3030/rooms/Bedroom - delete room
#[tokio::main]
async fn main() {
    dotenv().ok();

    let room_name = "Bedroom";
    let device_socket_name = "Socket";

    let smart_home = SmartHome::bulder().with_name("My home").build();
    let mut info_provider = DeviceInfoProvider::default();
    let socket = Socket::new(device_socket_name);
    if let Err(e) =
        info_provider.add_device(room_name, device_socket_name, DeviceType::Socket(socket))
    {
        println!("{}", e);
    };

    let repository = SqliteRepository::new().await.unwrap();
    let service = SmartHomeService::new(smart_home, info_provider, repository);
    let request_handler = Arc::new(Mutex::new(RequestHandler::new(service)));

    let routes = filters::api(request_handler);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
