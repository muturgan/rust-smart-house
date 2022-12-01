#![allow(clippy::needless_return)]

/**
 * - Дом имеет название и содержит несколько помещений.
 * - Библиотека позволяет запросить список помещений в доме.
 * - Помещение имеет уникальное название и содержит названия нескольких устройств.
 * - Устройство имеет уникальное в рамках помещения имя.
 * - Библиотека позволяет получать список устройств в помещении.
 * - Библиотека имеет функцию, возвращающую текстовый отчёт о состоянии дома.
 *      Эта функция принимает в качестве аргумента обобщённый тип, позволяющий получить текстовую информацию
 *      о состоянии устройства, для включения в отчёт. Эта информация должна предоставляться
 *      для каждого устройства на основе данных о положении устройства в доме: имени комнаты и имени устройства.
 *      Если устройство не найдено в источнике информации, то вместо текста о состоянии вернуть сообщение об ошибке.
 */
use ::std::collections::HashSet;

trait IReporter {
    fn create_report(&self) -> Result<String, String>;
}

// Пользовательские устройства:
struct SmartSocket {
    name: String,
}

trait IDevice: IReporter {
    fn get_name(&self) -> &String;
}

impl IReporter for SmartSocket {
    fn create_report(&self) -> Result<String, String> {
        let message = format!("Это умная розетка '{}'. Роботает штатно.", &self.name);
        return Ok(message);
    }
}

impl IDevice for SmartSocket {
    fn get_name(&self) -> &String {
        return &self.name;
    }
}

struct SmartThermometer {
    name: String,
}

impl IReporter for SmartThermometer {
    fn create_report(&self) -> Result<String, String> {
        let message = format!("Это умный термометр '{}'. Роботает штатно.", self.name);
        return Ok(message);
    }
}

impl IDevice for SmartThermometer {
    fn get_name(&self) -> &String {
        return &self.name;
    }
}

// Помещения

trait IRoom: IReporter {
    fn get_name(&self) -> &String;
    fn get_devices_names(&self) -> Vec<&String>;
}

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

// Умный дом
struct SmartHouse<'a> {
    name: String,
    rooms: Vec<&'a dyn IRoom>,
}

impl<'a> SmartHouse<'a> {
    fn new(name: String, rooms: Vec<&'a dyn IRoom>) -> Result<Self, String> {
        let rooms_ref = &rooms;

        let rooms_names = &rooms_ref.iter().map(|d| d.get_name()).collect::<Vec<_>>();

        let unique_rooms_names = rooms_names.iter().collect::<HashSet<_>>();

        if rooms_names.len() != unique_rooms_names.len() {
            return Err(String::from("Комнаты в доме имеют неуникальные названия"));
        }

        return Ok(Self { name, rooms });
    }

    fn _get_room_names(&self) -> Vec<&String> {
        return self.rooms.iter().map(|d| d.get_name()).collect::<Vec<_>>();
    }

    fn _get_room_devices_names(&self, room_name: &str) -> Result<Vec<&String>, String> {
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

        Ok(report)
    }
}

fn main() {
    // Инициализация устройств
    let socket1 = &SmartSocket {
        name: String::from("розетка для телевизора"),
    };
    let socket2 = &SmartSocket {
        name: String::from("розетка для аквариума"),
    };
    let thermo2 = &SmartThermometer {
        name: String::from("термометр для аквариума"),
    };
    let thermo3 = &SmartThermometer {
        name: String::from("термометр для самогонного аппарата"),
    };

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
