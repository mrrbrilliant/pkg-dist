use crate::structures::{configuration::Config, repository::Repo};

impl Config {
    pub fn new(
        repos: Vec<Repo>,
        production: bool,
        db_directory: String,
        cache_directory: String,
        local_db: String,
        registry: String,
        version_file: String,
        executable_ext: String,
        installation_target: String,
    ) -> Self {
        Self {
            repos: repos,
            production: production,
            db_directory: db_directory.to_string(),
            cache_directory: cache_directory.to_string(),
            local_db: local_db.to_string(),
            registry: registry.to_string(),
            version_file: version_file.to_string(),
            executable_ext: executable_ext.to_string(),
            installation_target: installation_target.to_string(),
        }
    }
}
