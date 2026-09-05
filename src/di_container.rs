use crate::storage::Storage;

pub struct DIContainer {
    pub storage: Box<dyn Storage>,
}
