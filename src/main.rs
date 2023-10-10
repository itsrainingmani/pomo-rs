use bpaf::*;
use color_eyre::eyre::Result;
use indicatif::{self, ProgressBar};
use std::sync::mpsc::{self};
use std::thread;
use std::time::Duration;

use console::{style, Emoji, Style, Term};

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

        Timer {
            time_min,
            time_sec: 60 * time_min,
            current_time: 0,
        }
    }
}

#[derive(Debug)]
pub enum UserInput {
    Cancel,
    Pause,
    Resume,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let term = Term::stdout();
    let tomato = Style::new().red().dim();
    println!(
        "{} {}",
        tomato.apply_to("Welcome to Pomo-rs "),
        Emoji("🍅", "")
    );
    term.set_title("Pomodoro Timer");
    println!(
        "{}",
        tomato.apply_to("| C -> Cancel | P -> Pause | R -> Resume |"),
    );

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
                    tx.send(UserInput::Cancel).unwrap();
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
