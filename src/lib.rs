use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// An entirely Rust based Pomodoro timer
/// that seamlessly integrates with Discord
///
/// Example:
///
/// $ pomo-rs <time>
pub struct Cli {
    #[structopt(default_value = "25")]
    pub time: String,
}

#[derive(Debug)]
pub struct Config {
    pub time_min: usize,
    pub time_sec: usize,
    pub current_time: usize,
}

impl Config {
    pub fn new(cli: Cli) -> Result<Config, &'static str> {
        let time_min = match cli.time.parse::<usize>() {
            Ok(t) => t,
            Err(_e) => return Err("Please enter a valid time in minutes"),
        };

        let time_sec: usize = 60 * time_min;
        let current_time = 0;

        Ok(Config {
            time_min,
            time_sec,
            current_time,
        })
    }
}
