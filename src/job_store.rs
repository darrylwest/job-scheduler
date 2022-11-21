/// JobStore
///
// use anyhow::{anyhow, Result};
// use anyhow::Result;
use log::info;
// use serde::Serialize;
// use std::hash::Hash;
use std::collections::HashMap;
use tokio::sync::mpsc;
// use tokio::sync::oneshot;

// use domain_keys::models::Model;

#[derive(Debug, Default, Clone)]
pub struct Job {
    pub id: String,
    pub name: String,
}

impl Job {
    pub fn new(name: &str) -> Job {
        Job {
            id: domain_keys::keys::RouteKey::create(),
            name: name.to_string(),
        }
    }
}

// type Callback: tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub enum Command {
    Insert(Job),
}

#[derive(Debug)]
pub struct JobStore {
    req_sender: mpsc::Sender<Command>,
}

impl JobStore {
    /// create and return a valid database; connect to redis
    pub async fn new() -> JobStore {
        let req_sender: mpsc::Sender<Command>;
        let mut req_receiver: mpsc::Receiver<Command>;
        (req_sender, req_receiver) = mpsc::channel(64);

        tokio::spawn(async move {
            let mut map: HashMap<String, Job> = HashMap::new();

            while let Some(cmd) = req_receiver.recv().await {
                info!("req recv: {:?}", cmd);
                match cmd {
                    Command::Insert(job) => {
                        info!("insert job: {:?}", job);

                        map.insert(job.id.to_string(), job.clone());
                    }
                }

                info!("map: {:?}", map);
            }

            req_receiver.close();
        });

        JobStore { req_sender }
    }

    /// clients get access to the broadcast channel to send requests
    pub fn request_channel(&self) -> mpsc::Sender<Command> {
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
