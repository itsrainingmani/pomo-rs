use bpaf::*;
use color_eyre::eyre::Result;
use indicatif::{self, ProgressBar};
use std::sync::mpsc::{self};
use std::sync::{Arc, Mutex};
use std::thread;

use tokio::time::{self, Duration, MissedTickBehavior};

use console::{Emoji, Style, Term};

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
/// An entirely Rust based Pomodoro timer
///
/// Run a timer for 10 mins (Long), 5 mins (Short),
/// or specify your own intervals.
///
/// While the terminal is running, press
/// "| c -> cancel | p -> pause | r -> resume |"
pub enum Args {
    /// 10 minutes
    Long,

    /// 5 minutes
    Short,

    Time {
        /// Specify your own time in minutes
        time: usize,
    },
}

#[derive(Debug, PartialEq)]
pub enum TimerState {
    NotStarted,
    Running,
    Paused,
    Canceled,
}

#[derive(Debug)]
pub enum UserInput {
    Cancel,
    Pause,
    Resume,
}

#[derive(Debug)]
pub struct Timer {
    pub time_min: usize,
    pub time_sec: usize,
    pub current_time: usize, // in seconds
    pub state: TimerState,
}

impl Timer {
    pub fn new(args: Args) -> Self {
        let time_min = match args {
            Args::Long => 10,
            Args::Short => 5,
            Args::Time { time } => time,
        };

        Timer {
            time_min,
            time_sec: 60 * time_min,
            current_time: 0,
            state: TimerState::NotStarted,
        }
    }
}

pub fn format_duration(seconds: usize) -> String {
    let mins = seconds / 60;
    let seconds = seconds % 60;

    format!("{} mins, {} seconds", mins, seconds)
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let term = Term::stdout();
    term.set_title("Pomodoro Timer");

    let mut timer = Timer::new(args().run());

    let tomato = Style::new().red().dim();
    println!(
        "{} {}",
        tomato.apply_to("Welcome to Pomo-rs "),
        Emoji("🍅", "")
    );
    println!(
        "{}\n",
        tomato.apply_to("| c -> cancel | p -> pause | r -> resume |"),
    );

    // println!(
    //     "Pomodoro Time in min: {}; in seconds: {}; current time: {}",
    //     config.time_min, config.time_sec, config.current_time
    // );

    let pb = ProgressBar::new(timer.time_sec as u64);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{prefix}\n{wide_bar:.green/red}\n{msg}")?,
    );

    let mut interval = time::interval(Duration::from_millis(1000));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        while let Ok(key) = term.read_key() {
            match key {
                console::Key::Char('c') => {
                    tx.send(UserInput::Cancel).unwrap();
                    thread::yield_now();
                }
                console::Key::Char('p') => {
                    tx.send(UserInput::Pause).unwrap();
                }
                console::Key::Char('r') => {
                    tx.send(UserInput::Resume).unwrap();
                }
                _ => {}
            }
        }
    });

    timer.state = TimerState::Running;
    loop {
        // try_recv doesnt block. handle a message if there is one
        if let Ok(cmd) = rx.try_recv() {
            match cmd {
                UserInput::Pause => {
                    if timer.state == TimerState::Running {
                        timer.state = TimerState::Paused;
                        pb.set_message("Timer Paused");
                    }
                }
                UserInput::Cancel => {
                    timer.state = TimerState::Canceled;
                    break;
                }
                UserInput::Resume => {
                    if timer.state == TimerState::Paused {
                        timer.state = TimerState::Running;
                        pb.set_message("");
                    }
                } // _ => println!("{:?}", cmd),
            }
        }
        if timer.state == TimerState::Paused {
            continue;
        }
        interval.tick().await;
        pb.inc(1);
        pb.set_prefix(format_duration(timer.current_time));
        timer.current_time += 1;

        if timer.current_time >= timer.time_sec {
            break;
        }
        // thread::sleep(Duration::from_millis(1000));
        // pb.inc(1);
    }
    let finish_msg = match timer.state {
        TimerState::Canceled => String::from("🍅 Canceled!"),
        _ => {
            pb.set_prefix(format_duration(timer.time_sec));
            String::from("🍅 Squashed!")
        }
    };
    pb.finish_with_message(tomato.apply_to(finish_msg).to_string());

    Ok(())
}
