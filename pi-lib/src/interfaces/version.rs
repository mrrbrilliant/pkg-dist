use crate::structures::version::Version;

impl Version {
    pub fn new(version: u32) -> Self {
        Self {
            version: version as u32,
        }
    }
}
