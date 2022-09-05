use iced::{
    button, Alignment, Button, Column, Container, Element, Length, Sandbox, Settings, Text,
};
use smart_home::network::TcpClient;

const CONNECTION_NOT_FOUND: &str = "Connection not found!";
const CONNECTION_SUCCESSFUL: &str = "Connection is successful!";
const SOCKET_ALREADY_CONNECT: &str = "Socket is already connect!";

fn main() -> iced::Result {
    SocketGui::run(Settings {
        window: iced::window::Settings {
            size: (350, 350),
            ..Default::default()
        },
        ..Default::default()
    })
}

struct SocketGui {
    client: Option<TcpClient>,
    text: String,
    button_connect: button::State,
    button_switch: button::State,
    button_status: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    Connect,
    Switch,
    Status,
}

impl Sandbox for SocketGui {
    type Message = Message;

    fn new() -> Self {
        SocketGui {
            client: None,
            text: String::new(),
            button_connect: button::State::default(),
            button_switch: button::State::default(),
            button_status: button::State::default(),
        }
    }

    fn title(&self) -> String {
        String::from("Smart socket GUI")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Connect => {
                if self.client.is_none() {
                    match TcpClient::connect("127.0.0.1:8080") {
                        Ok(client) => {
                            self.client = Some(client);
                            self.text = String::from(CONNECTION_SUCCESSFUL);
                        }
                        Err(error) => self.text = error.to_string(),
                    }
                } else {
                    self.text = String::from(SOCKET_ALREADY_CONNECT);
                }
            }
            Message::Switch => {
                if let Some(client) = &mut self.client {
                    match client.send_request("switch") {
                        Ok(response) => self.text = response,
                        Err(error) => self.text = error.to_string(),
                    }
                } else {
                    self.text = String::from(CONNECTION_NOT_FOUND);
                }
            }
            Message::Status => {
                if let Some(client) = &mut self.client {
                    match client.send_request("status") {
                        Ok(response) => self.text = response,
                        Err(error) => self.text = error.to_string(),
                    }
                } else {
                    self.text = String::from(CONNECTION_NOT_FOUND);
                }
            }
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let content = Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Alignment::Center)
            .push(
                Button::new(&mut self.button_connect, Text::new("Connect"))
                    .on_press(Message::Connect),
            )
            .push(
                Button::new(&mut self.button_switch, Text::new("Switch")).on_press(Message::Switch),
            )
            .push(
                Button::new(&mut self.button_status, Text::new("Status")).on_press(Message::Status),
            )
            .push(Text::new(&self.text).size(20));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
