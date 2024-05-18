use anyhow::Result;
use config::{Config, FileFormat, Map};
use dotenvy::dotenv;
use primitypes::contest::Language;

const CONFIGURATION_DIRECTORY: &str = "CONFIGURATION_DIRECTORY";
const CONFIGURATION_FILE: &str = "CONFIGURATION_FILE";

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub is_prod: bool,
    pub notifications: RedisSettings,
    pub cache: Option<RedisSettings>,
    pub rabbitmq: RabbitMqSettings,
    pub postgres: PostgresSettings,
    pub upstream: UpStreamSettings,
    pub evaluator: EvaluatorSettings,
    pub language: LanguageBinary,
}

#[derive(serde::Deserialize, Clone)]
pub struct RedisSettings {
    pub host: String,
    pub port: usize,
    pub channel: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct RabbitMqSettings {
    pub host: String,
    pub queue: String,
    pub consumer: String,
    pub port: usize,
}
#[derive(serde::Deserialize, Clone)]
pub struct PostgresSettings {
    pub host: String,
    pub user: String,
    pub database: String,
    pub port: usize,
    pub password: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct UpStreamSettings {
    pub uri: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct EvaluatorSettings {
    pub playground: String,
    pub resources: String,
    pub nsjail_config: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct LanguageBinary(pub Map<Language, CmdStr>);

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct CmdStr {
    pub path: String,
    pub args: Vec<String>,
    pub eval_type: EvaluationType,
    pub file_type: String,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub enum EvaluationType {
    #[default]
    Compiled,
    Interpreted,
    Java,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let (is_prod, config_dir, config_file);
    match dotenv() {
        Ok(_) => {
            is_prod = dotenvy::var("IS_PROD")
                .expect("IS_PROD is not set")
                .parse::<bool>()
                .unwrap();
            config_dir =
                dotenvy::var(CONFIGURATION_DIRECTORY).expect("CONFIGURATION_DIRECTORY is not set");
            config_file = dotenvy::var(CONFIGURATION_FILE).expect("CONFIGURATION_FILE is not set");
        },
        Err(_) => {
            is_prod = std::env::var("IS_PROD")
                .expect("IS_PROD is not set")
                .parse::<bool>()
                .unwrap();
            config_dir =
                std::env::var(CONFIGURATION_DIRECTORY).expect("CONFIGURATION_DIRECTORY is not set");
            config_file = std::env::var(CONFIGURATION_FILE).expect("CONFIGURATION_FILE is not set");
        },
    }

    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join(config_dir);

    let settings = Config::builder()
        .add_source(
            config::File::from(configuration_directory.join(config_file)).format(FileFormat::Yaml),
        )
        .build()?;

    settings
        .try_deserialize::<Settings>()
        .map(|s| Settings { is_prod, ..s })
}
