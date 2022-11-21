//!
//!

use anyhow::Result;
use log::info;
// use clap::{Parser, Subcommand}
use job_scheduler::config::Config;
use job_scheduler::job_store::{Command, Job, JobStore};
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::read_config("config/server-config.toml")?;
    config.start_logger()?;

    Config::write_pid_file();

    let store = JobStore::new().await;
    let mut event_channel = store.subscribe();

    tokio::spawn(async move {
        while let Ok(event) = event_channel.recv().await {
            info!("event: {:?}", event);
        }
    });

    let request_channel = store.request_channel();

    let job = Job::new("my job 100 name");
    let cmd = Command::Insert(job);
    let r = request_channel.send(cmd).await;
    info!("r1 {:?}", r);

    let job = Job::new("my job 200 name");
    let cmd = Command::Insert(job);
    let r = request_channel.send(cmd).await;
    info!("r1 {:?}", r);

    match signal::ctrl_c().await {
        Ok(()) => {
            println!("ctrl-c signal, save data and remove the pid file");

            Config::remove_pid_file();
        }
        Err(err) => {
            eprintln!("can't listen for ctrl-c: {:?}", err);
        }
    }

    Ok(())
}
