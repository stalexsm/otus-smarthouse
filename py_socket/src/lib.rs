use pyo3::prelude::*;

use socket_lib::{Command, Response, SmartSocketClient, SmartSocketError};

#[pyclass]
struct SmartClient {
    client: SmartSocketClient,
    state: Option<String>,
}

#[pymethods]
impl SmartClient {
    #[new]
    fn new() -> Self {
        let client = match SmartSocketClient::new("127.0.0.1:43212") {
            Ok(client) => client,
            Err(SmartSocketError::Io(e)) => panic!(
                "Клиент не смог достучаться до сервера. Нужно запустить `cargo run --example socket` {}",
                e
            ),
        };

        Self {
            client,
            state: None,
        }
    }

    fn switch_on(&mut self) {
        if let Ok(resp) = self.client.run_command(Command::TurnOn) {
            match resp {
                Response::Ok => self.state = Some(String::from("OK")),
                _ => self.state = Some(String::from("У вас не вышло включить розетку")),
            };
        }
    }

    fn switch_off(&mut self) {
        if let Ok(resp) = self.client.run_command(Command::TurnOff) {
            match resp {
                Response::Ok => self.state = Some(String::from("OK")),
                _ => self.state = Some(String::from("У вас не вышло выключить розетку")),
            };
        }
    }

    fn get_power(&mut self) {
        if let Ok(resp) = self.client.run_command(Command::GetPower) {
            match resp {
                Response::Power(pwr) => {
                    self.state = Some(format!("Напряжение в розетке: {}", pwr.clone()))
                }
                _ => {
                    self.state = Some(String::from(
                        "У вас не получилось получить напряжение в розетке",
                    ))
                }
            };
        }
    }

    fn is_enabled(&mut self) {
        if let Ok(resp) = self.client.run_command(Command::IsEnabled) {
            match resp {
                Response::Enabled => self.state = Some(format!("Розетка включена!")),
                Response::Disabled => self.state = Some(format!("Розетка выключена!")),
                _ => {
                    self.state = Some(String::from(
                        "У вас не получилось получить напряжение в розетке",
                    ))
                }
            };
        }
    }

    fn get_state(&self) -> Option<String> {
        self.state.clone()
    }
}

///
/// Модуль языка программирования Python.
///
#[pymodule]
#[pyo3(name = "pysocket")]
fn string_sum(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<SmartClient>()?;
    Ok(())
}
