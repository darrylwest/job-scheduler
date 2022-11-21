//!
//!

use anyhow::Result;
use log::info;
// use clap::{Parser, Subcommand}
use job_scheduler::config::Config;
use job_scheduler::db::Db;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::read_config("config/server-config.toml")?;
    config.start_logger()?;

    Config::write_pid_file();

    let db = Db::new().await;

    let sender = db.sender();
    let r = sender.send("my-command-1".to_string()).await;
    info!("r1 {:?}", r);

    let r = sender.send("my-command-2".to_string()).await;
    info!("r2 {:?}", r);

    let r = sender.send("my-command-3".to_string()).await;
    info!("r3 {:?}", r);

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
