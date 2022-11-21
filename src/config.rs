use anyhow::Result;
use log::{info, warn};
use serde::Deserialize;
use std::io::prelude::*;
use std::{
    fs::File,
    io::{BufReader, Read},
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Config {
    pub name: String,
    pub version: String,
    pub host: String,
    pub port: u16,
    pub logging_config: String,
    pub data_folder: String,
}

impl Config {
    // read and parse the config file
    pub fn read_config(filename: &str) -> Result<Config> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);

        let mut text = String::new();
        reader.read_to_string(&mut text)?;
        let config: Config = toml::from_str(&text).unwrap();

        info!(
            "config: {}, version: {}, host: {}, port: {}",
            config.name, config.version, config.host, config.port
        );

        Ok(config)
    }

    /// create and return a copy
    pub fn copy(&self) -> Config {
        Config {
            name: self.name.to_string(),
            version: self.version.to_string(),
            host: self.host.to_string(),
            port: self.port,
            logging_config: self.logging_config.to_string(),
            data_folder: self.data_folder.to_string(),
        }
    }

    /// start the logger
    pub fn start_logger(&self) -> Result<()> {
        log4rs::init_file(&self.logging_config, Default::default())?;
        info!("START THE SERVICE LOG: {}", "-".repeat(80));

        Ok(())
    }

    /// return the socket address that server listens on
    pub fn socket_address(&self) -> SocketAddr {
        let v4 = IpAddr::from_str(self.host.as_str()).expect("a good host name from config");

        SocketAddr::new(v4, self.port)
    }

    /// write the pid file
    pub fn write_pid_file() {
        let pid = std::process::id().to_string();
        info!("write pid {} to file: {}", pid, crate::SERVER_PID_FILE);
        let mut file = File::create(crate::SERVER_PID_FILE).expect("should open the file");
        file.write_all(pid.as_bytes())
            .expect("should write to the pid file")
    }

    /// remove the pid file on exit
    pub fn remove_pid_file() {
        info!("remove pid dfile: {}", crate::SERVER_PID_FILE);
        let resp = std::fs::remove_file(crate::SERVER_PID_FILE);
        if resp.is_err() {
            warn!("error removing pid: {:?}", resp);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let config = Config::read_config("tests/server-config.toml").unwrap();
        assert!(!config.name.is_empty());
        assert!(!config.data_folder.is_empty());
    }

    #[test]
    fn socket_address() {
        let config = Config::read_config("tests/server-config.toml").unwrap();
        let addr = config.socket_address();

        assert_eq!(format!("{}", addr), "127.0.0.1:28600");
    }

    #[test]
    fn write_remove_pid_file() {
        let pid = std::process::id().to_string();
        Config::write_pid_file();

        let mut file = File::open(crate::SERVER_PID_FILE).expect("pid file should exist");

        let mut buf = String::new();
        let resp = file.read_to_string(&mut buf);
        assert_eq!(resp.is_ok(), true);
        assert_eq!(buf, pid);

        Config::remove_pid_file();
        let result = File::open(crate::SERVER_PID_FILE);
        assert_eq!(result.is_err(), true);
    }
}
