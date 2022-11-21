use domain_keys::{
    keys::RouteKey, keys::TimeStampKey, models::Model, models::Status, models::Version,
};
/// Job models
///
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

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Job {
    pub topic: String,
    pub description: String,
    pub action: String,
    pub results: Option<String>,
    pub log: Vec<String>,
    pub errors: Vec<String>,
}

impl Job {
    pub fn new(topic: &str, action: &str) -> Job {
        Job {
            topic: topic.to_string(),
            description: String::new(),
            action: action.to_string(),
            results: None,
            log: Vec::new(),
            errors: Vec::new(),
        }
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
