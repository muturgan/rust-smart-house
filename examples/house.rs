#![allow(clippy::needless_return)]

use ::std::collections::HashSet;
use rust_smart_house::common::IReporter;
use rust_smart_house::devices::standard::{SmartSocket, SmartThermometer};
use rust_smart_house::devices::IDevice;
use rust_smart_house::house::SmartHouse;
use rust_smart_house::rooms::IRoom;

// Пользовательские комнаты могут как хранить устройства, так и заимствывать.
struct OwningRoom<'a> {
    name: String,
    devices: Vec<&'a dyn IDevice>,
}

impl<'a> OwningRoom<'a> {
    fn new(name: String, devices: Vec<&'a dyn IDevice>) -> Result<Self, String> {
        let devices_names = devices.iter().map(|d| d.get_name()).collect::<Vec<_>>();

        let unique_devices_names = devices_names.iter().collect::<HashSet<_>>();

        if devices_names.len() != unique_devices_names.len() {
            return Err(String::from(
                "Устройства в комнате имеют неуникальные названия",
            ));
        }

        return Ok(Self { name, devices });
    }
}

impl<'a> IRoom for OwningRoom<'a> {
    fn get_name(&self) -> &String {
        return &self.name;
    }

    fn get_devices_names(&self) -> Vec<&String> {
        return self
            .devices
            .iter()
            .map(|d| d.get_name())
            .collect::<Vec<_>>();
    }
}

impl<'a> IReporter for OwningRoom<'a> {
    fn create_report(&self) -> Result<String, String> {
        let mut report = format!(" * комната '{}':\n", self.name);

        for device in &self.devices {
            let device_report = device.create_report();

            if device_report.is_err() {
                return Err(format!(
                    "Возникла ошибка при формировании отчёта с устройством '{}' в комнате '{}': {}",
                    device.get_name(),
                    &self.name,
                    device_report.err().unwrap()
                ));
            }

            report.push_str(&format!("   - {}\n", device_report.ok().unwrap()));
        }

        return Ok(report);
    }
}

// Пользовательские комнаты могут как хранить устройства, так и заимствывать.
struct BorrowingRoom<'a> {
    name: String,
    devices: &'a Vec<&'a dyn IDevice>,
}

impl<'a> BorrowingRoom<'a> {
    fn new(name: String, devices: &'a Vec<&'a dyn IDevice>) -> Result<Self, String> {
        let devices_names = devices.iter().map(|d| d.get_name()).collect::<Vec<_>>();

        let unique_devices_names = devices_names.iter().collect::<HashSet<_>>();

        if devices_names.len() != unique_devices_names.len() {
            return Err(String::from(
                "Устройства в комнате имеют неуникальные названия",
            ));
        }

        return Ok(Self { name, devices });
    }
}

impl<'a> IRoom for BorrowingRoom<'a> {
    fn get_name(&self) -> &String {
        return &self.name;
    }

    fn get_devices_names(&self) -> Vec<&String> {
        return self
            .devices
            .iter()
            .map(|d| d.get_name())
            .collect::<Vec<_>>();
    }
}

impl<'a> IReporter for BorrowingRoom<'a> {
    fn create_report(&self) -> Result<String, String> {
        let mut report = format!(" * комната '{}':\n", self.name);

        for device in self.devices {
            let device_report = device.create_report();

            if device_report.is_err() {
                return Err(format!(
                    "Возникла ошибка при формировании отчёта с устройством '{}' в комнате '{}': {}",
                    device.get_name(),
                    &self.name,
                    device_report.err().unwrap()
                ));
            }

            report.push_str(&format!("   - {}\n", device_report.ok().unwrap()));
        }

        return Ok(report);
    }
}

fn main() {
    // Инициализация устройств
    let socket1 = &SmartSocket::new(String::from("розетка для телевизора"));
    let socket2 = &SmartSocket::new(String::from("розетка для аквариума"));
    let thermo2 = &SmartThermometer::new(String::from("термометр для аквариума"));
    let thermo3 = &SmartThermometer::new(String::from("термометр для самогонного аппарата"));

    let dv2 = vec![socket2 as &dyn IDevice, thermo2];
    let dv3: &Vec<&dyn IDevice> = &vec![thermo3];

    // Инициализация комнат
    let hall = &OwningRoom::new(String::from("Зал"), vec![socket1]).unwrap();
    let kitchen = &BorrowingRoom::new(String::from("Кухня"), &dv2).unwrap();
    let storage = &BorrowingRoom::new(String::from("Кладовка"), dv3).unwrap();

    // Инициализация дома
    let house =
        SmartHouse::new(String::from("Дом, милый дом"), vec![hall, kitchen, storage]).unwrap();

    // Выводим отчёты на экран:
    println!("Report #1: {}", house.create_report().unwrap());
    println!("===========================");
    println!("Report #2: {}", house.create_report().unwrap());
}
