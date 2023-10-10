use clap::{ArgGroup, Parser};
use color_eyre::eyre::Result;
use indicatif::{self, ProgressBar};
use std::thread;
use std::time::Duration;

use console::Term;

#[derive(Debug, Parser)]
#[command(version, author, about, long_about = None, group = ArgGroup::new("duration").required(true))]
/// An entirely Rust based Pomodoro timer
pub struct Args {
    /// 10 minutes
    #[arg(short, long, group = "duration")]
    long: bool,

    /// 5 minutes
    #[arg(short, long, group = "duration")]
    short: bool,

    /// Specify your own time in minutes
    #[arg(short, long, group = "duration")]
    time: usize,
}

#[derive(Debug)]
pub struct Timer {
    pub time_min: usize,
    pub time_sec: usize,
    pub current_time: usize,
}

impl Timer {
    pub fn new(args: Args) -> Self {
        let time_min = if args.long {
            10
        } else if args.short {
            5
        } else {
            args.time
        };

        let time_sec: usize = 60 * time_min;
        let current_time = 0;

        Timer {
            time_min,
            time_sec,
            current_time,
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let term = Term::stdout();
    term.write_line("Welcome to Pomo-rs 🍅")?;

    /*
    TODO: Display KeyPress commands to the user. This should be permanently displayed
    */

    let timer = Timer::new(Args::parse());
    // println!("{:?}", timer);

    // println!(
    //     "Pomodoro Time in min: {}; in seconds: {}; current time: {}",
    //     config.time_min, config.time_sec, config.current_time
    // );

    let pb = ProgressBar::new(timer.time_min as u64);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {wide_bar:.green/red} [{eta_precise}] {msg}")?,
    );

    // TODO: loop for processing commands by user

    // Why loop per minute? maybe should loop per second?
    for _ in 0..=timer.time_min {
        thread::sleep(Duration::from_millis(1000));
        pb.inc(1);
    }
    pb.finish_with_message("done");

    Ok(())
}
