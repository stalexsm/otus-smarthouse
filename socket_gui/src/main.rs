#![cfg_attr(not(debug_assertions), windows_subsystem = "window")]

use iced::widget::{button, column, container, row, text};
use iced::{executor, Application, Command, Element, Length, Settings, Theme};

use socket_lib::{Command as Message, Response, SmartSocketClient, SmartSocketError};

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Debug)]
struct App {
    client: SmartSocketClient,
    state: String,
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let client = match SmartSocketClient::new("127.0.0.1:43212") {
            Ok(client) => client,
            Err(SmartSocketError::Io(e)) => panic!(
                "Клиент не смог достучаться до сервера. Нужно запустить `cargo run --example socket` {}",
                e
            ),
        };

        (
            Self {
                client,
                state: String::from("Данных пока нет!"),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Управление умной розеткой - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        if let Ok(resp) = self.client.run_command(message.clone()) {
            match resp {
                Response::Ok => match message {
                    Message::TurnOn => self.state = String::from("Вы включили розетку"),
                    _ => self.state = String::from("Вы выключили розетку"),
                },
                Response::Enabled => self.state = String::from("Розетка включена"),
                Response::Disabled => self.state = String::from("Розетка выключена"),
                Response::Power(pwr) => self.state = format!("Напряжение в розетке: {}", pwr),
                Response::Unknown => self.state = String::from("Unknown"),
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let info = &self.state;

        let content: Element<_> = column![
            text(info),
            row![
                button("Включить розетку").on_press(Message::TurnOn),
                button("Выключить розетку").on_press(Message::TurnOff),
                button("Напряжение").on_press(Message::GetPower),
                button("Включена?").on_press(Message::IsEnabled),
            ]
            .spacing(10)
        ]
        .spacing(10)
        .into();

        container(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
