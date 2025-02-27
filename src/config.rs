use crate::prelude::*;
use std::env;
use std::path::PathBuf;

use std::io::{Read as _, Write as _};
use tokio::io::AsyncReadExt as _;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Config {
    pub email: Option<String>,
    pub base_url: Option<String>,
    pub root_certificate: Option<String>,
    pub identity_url: Option<String>,
    #[serde(default = "default_lock_timeout")]
    pub lock_timeout: u64,
    #[serde(default = "default_pinentry")]
    pub pinentry: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            email: Default::default(),
            base_url: Default::default(),
            identity_url: Default::default(),
            lock_timeout: default_lock_timeout(),
            pinentry: default_pinentry(),
            root_certificate: Default::default(),
        }
    }
}

pub fn default_lock_timeout() -> u64 {
    3600
}

pub fn default_pinentry() -> String {
    "pinentry".to_string()
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load() -> Result<Self> {
        let file = crate::dirs::config_file();
        let mut fh = std::fs::File::open(&file).map_err(|source| {
            Error::LoadConfig {
                source,
                file: file.clone(),
            }
        })?;
        let mut json = String::new();
        fh.read_to_string(&mut json)
            .map_err(|source| Error::LoadConfig {
                source,
                file: file.clone(),
            })?;
        let mut slf: Self = serde_json::from_str(&json)
            .map_err(|source| Error::LoadConfigJson { source, file })?;
        if slf.lock_timeout == 0 {
            log::warn!("lock_timeout must be greater than 0");
            slf.lock_timeout = default_lock_timeout();
        }
        Ok(slf)
    }

    pub async fn load_async() -> Result<Self> {
        let file = crate::dirs::config_file();
        let mut fh =
            tokio::fs::File::open(&file).await.map_err(|source| {
                Error::LoadConfigAsync {
                    source,
                    file: file.clone(),
                }
            })?;
        let mut json = String::new();
        fh.read_to_string(&mut json).await.map_err(|source| {
            Error::LoadConfigAsync {
                source,
                file: file.clone(),
            }
        })?;
        let mut slf: Self = serde_json::from_str(&json)
            .map_err(|source| Error::LoadConfigJson { source, file })?;
        if slf.lock_timeout == 0 {
            log::warn!("lock_timeout must be greater than 0");
            slf.lock_timeout = default_lock_timeout();
        }
        Ok(slf)
    }

    pub fn save(&self) -> Result<()> {
        let file = crate::dirs::config_file();
        // unwrap is safe here because Self::filename is explicitly
        // constructed as a filename in a directory
        std::fs::create_dir_all(file.parent().unwrap()).map_err(
            |source| Error::SaveConfig {
                source,
                file: file.clone(),
            },
        )?;
        let mut fh = std::fs::File::create(&file).map_err(|source| {
            Error::SaveConfig {
                source,
                file: file.clone(),
            }
        })?;
        fh.write_all(
            serde_json::to_string(self)
                .map_err(|source| Error::SaveConfigJson {
                    source,
                    file: file.clone(),
                })?
                .as_bytes(),
        )
        .map_err(|source| Error::SaveConfig { source, file })?;
        Ok(())
    }

    pub fn validate() -> Result<()> {
        let config = Self::load()?;
        if config.email.is_none() {
            return Err(Error::ConfigMissingEmail);
        }
        Ok(())
    }

    pub fn base_url(&self) -> String {
        self.base_url.clone().map_or_else(
            || "https://api.bitwarden.com".to_string(),
            |url| format!("{}/api", url.trim_end_matches('/')),
        )
    }

    fn get_home_dir() -> PathBuf {
        #[allow(deprecated)]
        let dir: PathBuf = match env::home_dir() {
            Some(path) => PathBuf::from(path),
            None => PathBuf::from(""),
        };
        dir
    }

    pub fn root_certificate(&self) -> String {
        let home_path: PathBuf = Self::get_home_dir();
        self.root_certificate.clone().map_or_else(
            || "".to_string(),
            |cert| cert.replace("$HOME", home_path.to_str().unwrap()),
        )
    }

    pub fn identity_url(&self) -> String {
        self.identity_url.clone().unwrap_or_else(|| {
            self.base_url.clone().map_or_else(
                || "https://identity.bitwarden.com".to_string(),
                |url| format!("{}/identity", url.trim_end_matches('/')),
            )
        })
    }

    pub fn server_name(&self) -> String {
        self.base_url
            .clone()
            .unwrap_or_else(|| "default".to_string())
    }
}
