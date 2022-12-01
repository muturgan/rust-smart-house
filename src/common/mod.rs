pub trait IReporter {
    fn create_report(&self) -> Result<String, String>;
}
