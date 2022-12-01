#![allow(clippy::needless_return)]

use crate::common::IReporter;
use crate::rooms::IRoom;
use ::std::collections::HashSet;

pub struct SmartHouse<'a> {
    name: String,
    rooms: Vec<&'a dyn IRoom>,
}

impl<'a> SmartHouse<'a> {
    pub fn new(name: String, rooms: Vec<&'a dyn IRoom>) -> Result<Self, String> {
        let rooms_ref = &rooms;

        let rooms_names = &rooms_ref.iter().map(|d| d.get_name()).collect::<Vec<_>>();

        let unique_rooms_names = rooms_names.iter().collect::<HashSet<_>>();

        if rooms_names.len() != unique_rooms_names.len() {
            return Err(String::from("Комнаты в доме имеют неуникальные названия"));
        }

        return Ok(Self { name, rooms });
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }

    pub fn get_room_names(&self) -> Vec<&String> {
        return self.rooms.iter().map(|d| d.get_name()).collect::<Vec<_>>();
    }

    pub fn get_room_devices_names(&self, room_name: &str) -> Result<Vec<&String>, String> {
        let target_room = self.rooms.iter().find(|d| d.get_name() == room_name);

        return match target_room {
            None => Err(format!("В доме нет комнаты с названием '{}'", room_name)),
            Some(r) => Ok(r.get_devices_names()),
        };
    }
}

impl<'a> IReporter for SmartHouse<'a> {
    fn create_report(&self) -> Result<String, String> {
        let mut report = format!("Отчёт по дому '{}':\n", self.name);

        for room in &self.rooms {
            let room_report = room.create_report();

            if room_report.is_err() {
                return Err(format!(
                    "Возникла ошибка при формировании отчёта по дому '{}': {}",
                    &self.name,
                    room_report.err().unwrap()
                ));
            }

            report.push_str(&room_report.ok().unwrap());
        }

        return Ok(report);
    }
}
