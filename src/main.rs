use anyhow::Context;
use anyhow::Result;
use config::Configuration;
use dotenvy::dotenv;
use rand;
use rand::distributions::Distribution;
use statrs::distribution::Poisson;
use std::process::{Child, Command};
use std::time::Duration;

mod config;

fn start_tinkr(config: &Configuration, handles: &mut Vec<Child>) -> Result<()> {
    log::info!("Starting all tinkr instances");
    for exe in config.executeables.clone() {
        handles.push(
            Command::new("sh")
                .args(["-c", &format!("./{}", exe)])
                .spawn()
                .context("Failed to create handle to tinkr")?,
        );
        log::info!("Tinkr {} started", exe);
    }
    Ok(())
}

fn stop_tinkr() -> Result<()> {
    log::info!("Stopping all tinkr instances");
    Command::new("sh")
        .args(["pkill", "-f", "Wow"])
        .spawn()
        .context("Failed to kill wow")?;

    log::info!("Killed all world of warcraft instances");

    Ok(())
}

fn main() -> Result<()> {
    dotenv().ok();

    let config = config::parse();
    let mut tinkr_handles: Vec<Child> = vec![];

    let mut r = rand::thread_rng();
    let short = Poisson::new(config.short_break.into()).unwrap();
    let long = Poisson::new(config.long_break.into()).unwrap();

    loop {
        start_tinkr(&config, &mut tinkr_handles);

        let next_break = long.sample(&mut r);
        log::info!("Next break will be in {} minutes", next_break);
        std::thread::sleep(Duration::from_secs(60 * next_break as u64));
        let break_length = short.sample(&mut r);
        log::info!("Taking a break for {} minutes", break_length);
        stop_tinkr();
        std::thread::sleep(Duration::from_secs(60 * break_length as u64))
    }
}
