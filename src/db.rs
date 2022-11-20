/// Db
///
use anyhow::{anyhow, Result};
use log::info;
use serde::Serialize;
// use std::hash::Hash;
use tokio::sync::broadcast;
use tokio::sync::broadcast::{Receiver, Sender};

// use domain_keys::models::Model;

#[derive(Debug)]
pub struct Db {
    sender: Sender<String>,
}

impl Db {
    /// create and return a valid database; connect to redis
    pub fn new() -> Db<T> {
        let (sender, _receiver) = broadcast::channel(16);

        Db { data, sender }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let db: Db = Db::new();

        assert!(true);
    }
}
