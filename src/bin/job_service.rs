//!
//!

use anyhow::Result;
use log::{debug, info};
// use clap::{Parser, Subcommand}
use job_scheduler::config::Config;
use job_scheduler::job_store::{Command, JobStore};
use job_scheduler::models::Job;
use tokio::signal;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::read_config("config/server-config.toml")?;
    config.start_logger()?;

    Config::write_pid_file();

    let store = JobStore::new().await;
    let mut event_channel = store.subscribe();

    tokio::spawn(async move {
        while let Ok(event) = event_channel.recv().await {
            debug!("event: {:?}", event);
        }
    });

    let request_channel = store.request_channel();

    let job = Job::new("my job 100 name", "no-op");
    let model = Job::create_model(&job);
    let cmd = Command::Insert(model);
    let r = request_channel.send(cmd).await;
    debug!("r1 {:?}", r);

    let job = Job::new("my job 200 name", "no-op");
    let model = Job::create_model(&job);
    let key = model.key.to_string();
    let cmd = Command::Insert(model.clone());
    let r = request_channel.send(cmd).await;
    debug!("r1 {:?}", r);

    let cmd = Command::Find(model.key.to_string());
    let r = request_channel.send(cmd).await;
    debug!("r1 {:?}", r);

    let cmd = Command::Find("bad-job-id".to_string());
    let r = request_channel.send(cmd).await;
    debug!("r1 {:?}", r);

    let cmd = Command::ListKeys(0, 100);
    let r = request_channel.send(cmd).await;
    debug!("r1 {:?}", r);

    // this is the sequece that should be followed: 1) create the onshot channel 
    let (tx, rx) = oneshot::channel();

    // 2) define the callback task with join
    let join = tokio::task::spawn(async move {
        let cbd = rx.await;
        info!("CALLBACK data: {:?} {}", cbd, String::from("*").repeat(25));
        match cbd {
            Ok(data) => data,
            _ => None,
        }
    });

    // 3) create and send the request message
    let cmd = Command::Callback(key, tx);
    let r = request_channel.send(cmd).await;
    info!("CALLBACK call result {:?}", r);

    // 4) wait for the join handle to return results
    let data = join.await?;
    info!("CALLBACK data: {:?} {}", data, String::from("~").repeat(25));

    match signal::ctrl_c().await {
        Ok(()) => {
            info!("ctrl-c signal, save data and remove the pid file");

            Config::remove_pid_file();
        }
        Err(err) => {
            eprintln!("can't listen for ctrl-c: {:?}", err);
        }
    }

    Ok(())
}
