use bpaf::*;
use color_eyre::eyre::Result;
use indicatif::{self, ProgressBar};
use std::sync::mpsc::{self};
use std::thread;
use std::time::Duration;

use console::Term;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
/// An entirely Rust based Pomodoro timer
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

#[derive(Debug)]
pub struct Timer {
    pub time_min: usize,
    pub time_sec: usize,
    pub current_time: usize,
}

impl Timer {
    pub fn new(args: Args) -> Self {
        let time_min = match args {
            Args::Long => 10,
            Args::Short => 5,
            Args::Time { time } => time,
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

#[derive(Debug)]
pub enum Command {
    Cancel,
    Pause,
    Resume,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let term = Term::stdout();
    term.write_line("Welcome to Pomo-rs 🍅")?;
    term.set_title("Pomodoro Timer");
    term.write_line("| C -> Cancel | P -> Pause | R -> Resume |")?;

    let timer = Timer::new(args().run());

    // println!(
    //     "Pomodoro Time in min: {}; in seconds: {}; current time: {}",
    //     config.time_min, config.time_sec, config.current_time
    // );

    let pb = ProgressBar::new(timer.time_sec as u64);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {wide_bar:.green/red} [{eta_precise}]\n{msg}")?,
    );

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || loop {
        if let Ok(key) = term.read_key() {
            match key {
                console::Key::Char('c') => {
                    tx.send(Command::Cancel).unwrap();
                }
                console::Key::Char('p') => {
                    tx.send(Command::Pause).unwrap();
                }
                console::Key::Char('r') => {
                    tx.send(Command::Resume).unwrap();
                }
                _ => {}
            }
        }
    });

    let handle = thread::spawn(move || {
        for _ in 0..=timer.time_sec {
            // try_recv doesnt block. handle a message if there is one
            if let Ok(cmd) = rx.try_recv() {
                match cmd {
                    // Command::Pause => todo!(),
                    // Command::Cancel => todo!(),
                    // Command::Resume => todo!(),
                    _ => println!("{:?}", cmd),
                }
            }
            thread::sleep(Duration::from_millis(1000));
            pb.inc(1);
        }
        pb.finish_with_message("🍅 squashed!");
    });

    handle.join().unwrap();

    Ok(())
}
