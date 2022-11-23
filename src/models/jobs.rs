use crate::models::run_at::RunAt;
/// Job models
///
// use anyhow::Result;
use domain_keys::{
    keys::RouteKey, keys::TimeStampKey, models::Model, models::Status, models::Version,
};
use log::info;
use serde::{Deserialize, Serialize};

// use domain_keys::models::Model;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct JobEvent {
    pub mid: String,
    pub message: String,
    pub model: Option<Model<Job>>,
}

impl JobEvent {
    /// create a new job
    pub fn new(message: &str, model: Option<Model<Job>>) -> JobEvent {
        JobEvent {
            mid: TimeStampKey::create(),
            message: message.to_string(),
            model,
        }
    }
}

/// Job struct is designed to be serializable enable saving to disk or database actions are run
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Job {
    pub topic: String,
    pub description: String,
    pub run_at: Option<RunAt>,
    pub action: String,          // an OS Exec command with params
    pub results: Option<String>, // could be a simple string, comma delimited list, or json blob (why not Any?)
    pub log: Vec<String>,
    pub errors: Vec<String>,
}

impl Job {
    /// create a new job with topic and action
    pub fn new(topic: &str, action: &str) -> Job {
        Job {
            topic: topic.to_string(),
            description: String::new(),
            run_at: None,
            action: action.to_string(),
            results: None,
            log: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// create the job with topic, action and a run at time definition
    pub fn with_run_at(topic: &str, action: &str, run_at: RunAt) -> Job {
        let mut job = Job::new(topic, action);
        job.run_at = Some(run_at);

        job
    }

    /// create a new job wrapper model
    pub fn create_model(job: &Job) -> Model<Job> {
        let hash = Model::calc_hash(job);
        let key = RouteKey::create();
        let version = Version::new(hash);
        let status = Status::New(0);

        let model: Model<Job> = Model::create_model(key, &version, &status, job);

        info!("{:?}", model);

        model
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let job = Job::new("my test job", "backup");
        let model = Job::create_model(&job);

        assert_eq!(model.value, job);
        let json = serde_json::to_string(&model).unwrap();
        println!("json: {}", json);

        let jmodel: Model<Job> = serde_json::from_str(json.as_str()).unwrap();
        assert_eq!(model, jmodel);
    }
}
