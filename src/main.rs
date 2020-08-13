use indicatif;
use pomo_rs::{Cli, Config};
use std::process;
use std::thread;
use std::time;
use structopt::StructOpt;

fn main() {
    println!("Welcome to Pomo-rs");

    let config = Config::new(Cli::from_args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    println!(
        "Pomodoro Time in min: {}; in seconds: {}; current time: {}",
        config.time_min, config.time_sec, config.current_time
    );

    let pb = indicatif::ProgressBar::new(config.time_min as u64);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {wide_bar:.green/red} [{eta_precise}] {msg}"),
    );
    for i in 0..=config.time_min {
        thread::sleep(time::Duration::from_secs(1));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}
