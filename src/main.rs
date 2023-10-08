use bpaf::Bpaf;
use color_eyre::eyre::Result;
use indicatif;
use std::process;
use std::thread;
use std::time;

#[derive(Clone, Debug, Bpaf)]
#[bpaf(options, version)]
/// An entirely Rust based Pomodoro timer
struct Opts {
    /// 10 minutes
    pub long: bool,

    /// 5 minutes
    pub short: bool,

    /// Specify your own time in minutes
    pub time: usize,
}

#[derive(Debug)]
pub struct Config {
    pub time_min: usize,
    pub time_sec: usize,
    pub current_time: usize,
}

impl Config {
    // pub fn new(opts: Opts) -> Result<Config> {
    //     let time_minutes = opts::Ti

    //     let time_sec: usize = 60 * time_min;
    //     let current_time = 0;

    //     Ok(Config {
    //         time_min,
    //         time_sec,
    //         current_time,
    //     })
    // }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("Welcome to Pomo-rs");

    println!("{:?}", opts().run());

    // let config = Config::new(opts().run()).unwrap_or_else(|err| {
    //     eprintln!("{}", err);
    //     process::exit(1);
    // });

    // println!(
    //     "Pomodoro Time in min: {}; in seconds: {}; current time: {}",
    //     config.time_min, config.time_sec, config.current_time
    // );

    // let pb = indicatif::ProgressBar::new(config.time_min as u64);
    // pb.set_style(
    //     indicatif::ProgressStyle::default_bar()
    //         .template("[{elapsed_precise}] {wide_bar:.green/red} [{eta_precise}] {msg}")?,
    // );

    // for _ in 0..=config.time_min {
    //     thread::sleep(time::Duration::from_secs(1));
    //     pb.inc(1);
    // }
    // pb.finish_with_message("done");

    Ok(())
}
