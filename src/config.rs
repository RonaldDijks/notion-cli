use serde::{Deserialize, Serialize};

const CONFIG_DIR_NAME: &str = "notion-cli";
const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Debug)]
pub enum Error {
    NoConfigDir,
    Json { inner: serde_json::Error },
    Fs { inner: std::io::Error },
    Utf8 { inner: std::str::Utf8Error },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub secret: String,
    pub database_id: String,
}

impl Config {
    pub fn new(secret: String, database_id: String) -> Self {
        Self {
            secret,
            database_id,
        }
    }

    pub fn try_read() -> Result<Option<Config>, Error> {
        let config_file_path = dirs::config_dir()
            .ok_or(Error::NoConfigDir)?
            .join(CONFIG_DIR_NAME)
            .join(CONFIG_FILE_NAME);

        if let Ok(bytes) = std::fs::read(config_file_path) {
            let text = std::str::from_utf8(&bytes).map_err(|inner| Error::Utf8 { inner })?;
            let config: Self = serde_json::from_str(text).map_err(|inner| Error::Json { inner })?;
            Ok(Some(config))
        } else {
            Ok(None)
        }
    }

    pub fn write(&self) -> Result<(), Error> {
        let config_dir = dirs::config_dir()
            .ok_or(Error::NoConfigDir)?
            .join(CONFIG_DIR_NAME);
        std::fs::create_dir_all(&config_dir).map_err(|inner| Error::Fs { inner })?;
        let config_file = config_dir.join(CONFIG_FILE_NAME);
        let json = serde_json::to_string(&self).map_err(|inner| Error::Json { inner })?;
        std::fs::write(config_file, json).map_err(|inner| Error::Fs { inner })?;
        Ok(())
    }
}
