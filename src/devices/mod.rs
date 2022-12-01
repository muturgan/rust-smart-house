use crate::common::IReporter;

pub mod standard;

pub trait IDevice: IReporter {
    fn get_name(&self) -> &String;
}
