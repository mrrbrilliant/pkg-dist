use crate::structures::repository::Repo;

impl Repo {
    pub fn new(name: String, address: String) -> Self {
        Self {
            name: name.to_string(),
            address: address.to_string(),
        }
    }
}
