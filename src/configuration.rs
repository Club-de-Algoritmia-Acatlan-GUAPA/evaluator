use config::Config;

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub application_port: u16,
    pub queue: DatabaseSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool
}

impl DatabaseSettings {
    pub fn connection_url(&self) -> String {
        if self.password.len() == 0 {
            return format!(
                "postgres://{}@{}:{}/{}",
                self.username, self.host, self.port, self.database_name
            );
        }
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.as_str())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        let options = self.without_db().database(&self.database_name);
        options
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialise our configuration reader
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    let settings = Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .build()?;

    settings.try_deserialize()
}

#[cfg(test)]
mod tests {
    use crate::configuration::DatabaseSettings;

    #[test]
    fn test_connection_url() {
        let config = DatabaseSettings {
            username: "yollotlfe".to_string(),
            password: "".to_string(),
            port: 8000,
            host: "localhost".to_string(),
            database_name: "juezguapa".to_string(),
            require_ssl: true,
        };
        assert_eq!(
            config.connection_url(),
            "postgres://yollotlfe@localhost:8000/juezguapa".to_string()
        );

        let config = DatabaseSettings {
            username: "yollotlfe".to_string(),
            password: "lolo".to_string(),
            port: 8000,
            host: "127.0.0.1".to_string(),
            database_name: "juezguapa".to_string(),
            require_ssl: true,
        };

        assert_eq!(
            config.connection_url(),
            "postgres://yollotlfe:lolo@127.0.0.1:8000/juezguapa".to_string()
        );
    }
}
