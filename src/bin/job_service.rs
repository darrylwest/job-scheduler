//!
//!

use anyhow::Result;
use log::info;
// use clap::{Parser, Subcommand}
use job_scheduler::config::Config;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::read_config("config/server-config.toml")?;
    config.start_logger()?;

    Config::create_pid_file();

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