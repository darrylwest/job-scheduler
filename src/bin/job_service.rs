//!
//!

use anyhow::Result;
use log::info;
// use clap::{Parser, Subcommand}
use job_scheduler::config::Config;
use job_scheduler::db::{Command, Db, Job};
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::read_config("config/server-config.toml")?;
    config.start_logger()?;

    Config::write_pid_file();

    let db = Db::new().await;
    let request_channel = db.request_channel();

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
