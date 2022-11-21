/// JobStore.  A lock-less, thread safe in-memory data store implemented with messaging.
///
// use anyhow::Result;
use log::{error, info};
// use serde::Serialize;
use crate::models::{Job, JobEvent};
use domain_keys::models::Model;
use hashbrown::HashMap;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

// type Callback: tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub enum Command {
    Insert(Model<Job>),
    Find(String),
    Remove(String),
    ListKeys(usize, usize), // offset, limit
}

#[derive(Debug)]
pub struct JobStore {
    req_sender: mpsc::Sender<Command>,
    broadcaster: broadcast::Sender<JobEvent>,
}

impl JobStore {
    /// create and return a valid database; connect to redis
    pub async fn new() -> JobStore {
        let req_sender: mpsc::Sender<Command>;
        let mut req_receiver: mpsc::Receiver<Command>;

        (req_sender, req_receiver) = mpsc::channel(64);

        let broadcaster: broadcast::Sender<JobEvent>;
        let _subscriber: broadcast::Receiver<JobEvent>;
        (broadcaster, _subscriber) = broadcast::channel(64);

        let event_tx = broadcaster.clone();

        tokio::spawn(async move {
            let mut map: HashMap<String, Model<Job>> = HashMap::new();

            while let Some(cmd) = req_receiver.recv().await {
                info!("req recv: {:?}", cmd);
                match cmd {
                    Command::Insert(job) => {
                        map.insert(job.key.to_string(), job.clone());

                        let msg = format!("inserted job, id: {}", job.key);
                        info!("{msg}");

                        let event = JobEvent::new(&msg, Some(job.clone()));
                        fire(&event_tx, event);
                    }
                    Command::Find(key) => {
                        let event = if let Some(job) = map.get(&key) {
                            let msg = format!("found job id: {}", job.key);
                            JobEvent::new(&msg, Some(job.clone()))
                        } else {
                            let msg = format!("job not found for id: {}", key);
                            JobEvent::new(&msg, None)
                        };

                        fire(&event_tx, event);
                    }
                    Command::Remove(key) => {
                        let event = if let Some(job) = map.remove(&key) {
                            JobEvent::new("job removed", Some(job))
                        } else {
                            JobEvent::new("job not found", None)
                        };

                        fire(&event_tx, event);
                    }
                    Command::ListKeys(offset, limit) => {
                        let mut keys = String::new();
                        for key in map.keys().skip(offset).take(limit) {
                            if !keys.is_empty() {
                                keys.push(',');
                            }
                            keys.push_str(key);
                        }

                        let mut job = Job::new("keys", "");
                        job.results = Some(keys);
                        let model = Job::create_model(&job);
                        let event = JobEvent::new("list keys", Some(model));

                        fire(&event_tx, event);
                    }
                }

                // broadcast the job event
                fn fire(tx: &broadcast::Sender<JobEvent>, event: JobEvent) {
                    if tx.receiver_count() > 0 && tx.send(event).is_err() {
                        error!("event channel send error");
                    }
                }

                info!("map: {:?}", map);
            }

            req_receiver.close();
        });

        JobStore {
            req_sender,
            broadcaster,
        }
    }

    /// clients get access to the broadcast channel to send requests
    pub fn request_channel(&self) -> mpsc::Sender<Command> {
        self.req_sender.clone()
    }

    /// subscribe to job events
    pub fn subscribe(&self) -> broadcast::Receiver<JobEvent> {
        self.broadcaster.subscribe()
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
