use std::collections::HashMap;

/// Db
///
// use anyhow::{anyhow, Result};
use log::info;
// use serde::Serialize;
// use std::hash::Hash;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

// use domain_keys::models::Model;

#[derive(Debug)]
pub struct Db {
    req_sender: Sender<String>,
}

impl Db {
    /// create and return a valid database; connect to redis
    pub async fn new() -> Db {
        let (req_sender, mut req_receiver) = mpsc::channel(32);

        tokio::spawn(async move {
            let mut map: HashMap<String, String> = HashMap::new();

            while let Some(cmd) = req_receiver.recv().await {
                info!("req recv: {}", cmd);
                map.insert(cmd, "my value".to_string());
                info!("map: {:?}", map);
            }

            req_receiver.close();
        });

        Db { req_sender }
    }

    /// clients get access to the broadcast channel to send requests
    pub fn sender(&self) -> Sender<String> {
        self.req_sender.clone()
    }

}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn init() {
        assert!(true);
    }
}
