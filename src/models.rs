/// Job models
/// 

use domain_keys::keys::{RouteKey, TimeStampKey};

// use domain_keys::models::Model;

#[derive(Debug, Default, Clone)]
pub struct JobEvent {
    pub mid: String,
    pub message: String,
    pub job: Option<Job>,
}

impl JobEvent {
    /// create a new job
    pub fn new(message: &str, job: Option<Job>) -> JobEvent {
        JobEvent {
            mid: TimeStampKey::create(),
            message: message.to_string(),
            job,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Job {
    pub id: String,
    pub name: String,
}

impl Job {
    pub fn new(name: &str) -> Job {
        Job {
            id: RouteKey::create(),
            name: name.to_string(),
        }
    }
}
