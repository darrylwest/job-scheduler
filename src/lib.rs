#![doc = include_str!("../README.md")]

pub mod config;
pub mod job_store;
// pub mod jobs;
// pub mod session_store;

pub const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APIKEY: (&str, &str) = ("apikey", "3678fde6af84c28f93667fd9623ae6df");
pub const SERVER_PID_FILE: &str = "job-scheduler.pid";
pub const CLIENT_SESSION_KEY: &str = "client-session";
