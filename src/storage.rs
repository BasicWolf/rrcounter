use std::error::Error;

pub trait Storage {
    fn get_initial_value(&self) -> i64;
    fn persist_counter(&self, counter: i64) -> Result<(), Box<dyn Error>>;
}
