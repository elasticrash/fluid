use anyhow::Result;
use crossbeam_channel::{bounded, select, tick, Receiver};
use std::env;
use std::time::Duration;

use fluid_common::config::read;
use fluid_db::{configuration::Configuration, set_up_db};

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let config: Configuration = if args.len() > 1 && args[1] == "-f" {
        read(&args[2])
    } else {
        read("config.json")
    };

    let db = match set_up_db(&config).await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    let ctrl_c_events = ctrl_channel()?;
    let ticks = tick(Duration::from_secs(1));

    loop {
        select! {
            recv(ticks) -> _ => {
                println!("working!");
            }
            recv(ctrl_c_events) -> _ => {
                println!();
                println!("Goodbye!");
                break;
            }
        }
    }

    Ok(())
}
