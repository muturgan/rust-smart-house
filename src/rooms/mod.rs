use crate::common::IReporter;

pub trait IRoom: IReporter {
    fn get_name(&self) -> &String;
    fn get_devices_names(&self) -> Vec<&String>;
}
