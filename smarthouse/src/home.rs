use crate::devices::{Device, SmartSocket, SmartThermometer};
use crate::rooms::Room;
use std::collections::HashMap;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Clone)]
pub struct SmartHome {
    pub id: Uuid,
    pub name: String,
    pub rooms: HashMap<Uuid, Room>,
}

impl Display for SmartHome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}\nHome: {}", &self.id, &self.name)
    }
}

impl SmartHome {
    pub fn new(name: String) -> Self {
        SmartHome {
            id: Uuid::new_v4(),
            name,
            rooms: HashMap::new(),
        }
    }

    pub fn get_rooms(&self) -> Vec<&Room> {
        // Попучение списка комнат
        self.rooms.values().collect()
    }

    pub fn add_room(&mut self, room: Room) {
        // Добавление комнаты
        self.rooms.insert(room.id, room);
    }

    pub fn delete_room(&mut self, room: Room) {
        // Удаление комнаты
        self.rooms.remove(&room.id);
    }

    pub fn devices(&self, room_id: &Uuid) -> Option<Vec<&Device>> {
        // Метод для попучения списка девайсов в комнате
        if let Some(target_room) = self.rooms.get(room_id) {
            let devices: Vec<&Device> = target_room.devices.values().collect();
            Some(devices)
        } else {
            None
        }
    }

    pub fn create_report<T: DeviceInfoProvider>(&self, provider: &T) -> String {
        // Метод для формирования отчета в доме по комнатам и устройствам
        let mut report = String::new();

        report.push_str(&format!("{}\n", &self));
        for room in &self.get_rooms() {
            if let Some(devices) = self.devices(&room.id) {
                for device in devices.iter() {
                    report.push_str(&format!("{}\n", provider.get_device_info(room, device)));
                }
                report.push('\n');
            }
        }
        report
    }
}

pub struct OwningDeviceInfoProvider {
    pub socket: SmartSocket,
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a SmartSocket,
    pub thermo: &'b SmartThermometer,
}

pub trait DeviceInfoProvider {
    fn get_device_info(&self, room: &Room, device: &Device) -> String {
        let mut report = format!("{}\n", room);
        match device {
            Device::Socket(s) => report.push_str(&format!("{}\n", s)),
            Device::Thermometer(s) => report.push_str(&format!("{}\n", s)),
        }
        report
    }
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {}
impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_smarthome() {
        let house = SmartHome::new(String::from("My Dom"));

        assert_eq!(house.name, "My Dom");
    }

    #[test]
    fn add_room_in_smarthome() {
        let room = Room::new(String::from("My Room"));

        let mut house = SmartHome::new(String::from("My Dom"));
        house.add_room(room);

        assert_eq!(house.rooms.len(), 1);
    }

    #[test]
    fn delete_room_in_smarthome() {
        let room = Room::new(String::from("My Room"));

        let mut house = SmartHome::new(String::from("My Dom"));
        house.add_room(room.clone());
        house.delete_room(room);

        assert_eq!(house.get_rooms().len(), 0);
    }

    #[test]
    fn smarthome_get_rooms() {
        let room = Room::new(String::from("My Room"));

        let mut house = SmartHome::new(String::from("My Dom"));
        house.add_room(room);

        assert_eq!(house.get_rooms().len(), 1);
    }

    #[test]
    fn smarthome_room_get_devices() {
        let socket = SmartSocket::new(String::from("Socket"));
        let mut room = Room::new(String::from("My Room"));

        room.add_device(Device::Socket(socket));

        let mut house = SmartHome::new(String::from("My Dom"));
        house.add_room(room.clone());

        let devices = house.devices(&room.id).unwrap();

        assert_eq!(devices.len(), 1);
    }

    #[test]
    fn smarthome_create_report_owning() {
        let socket = SmartSocket::new(String::from("My Socket"));
        let mut room = Room::new(String::from("My Room"));

        room.add_device(Device::Socket(socket.clone()));

        let mut house = SmartHome::new(String::from("My Dom"));
        house.add_room(room.clone());

        let info_provider = OwningDeviceInfoProvider { socket };

        let report = house.create_report(&info_provider);

        assert!(!report.is_empty())
    }

    #[test]
    fn smarthome_create_report_borrowing() {
        let socket = SmartSocket::new(String::from("My Socket"));
        let thermo = SmartThermometer::new(String::from("My Thermometer"));
        let mut room = Room::new(String::from("My Room"));

        room.add_device(Device::Socket(socket.clone()));
        room.add_device(Device::Thermometer(thermo.clone()));

        let mut house = SmartHome::new(String::from("My Dom"));
        house.add_room(room.clone());

        let info_provider = BorrowingDeviceInfoProvider {
            socket: &socket,
            thermo: &thermo,
        };

        let report = house.create_report(&info_provider);

        assert!(!report.is_empty())
    }
}
