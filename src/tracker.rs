use colored::{Color, Colorize};

use crate::config::{Config, LogLevel};
use std::time::{Duration, Instant};

pub struct Tracker<'a> {
    start_time: Instant,
    config: &'a Config,
    title: &'a str,
    color: Color,
    title_list: Vec<&'a str>,
    opts: Option<TrackerOpts>,
}

#[derive(Debug)]
pub struct TrackerOpts {
    pub write_once: Option<bool>,
}

impl<'a> Tracker<'a> {
    pub fn new(config: &'a Config, color: Color, opts: Option<TrackerOpts>) -> Self {
        Tracker {
            config,
            title: "",
            start_time: Instant::now(),
            color,
            title_list: vec![],
            opts,
        }
    }

    pub fn start(&mut self, title: &'a str) {
        self.start_time = Instant::now();
        self.title = title;
        self.title_list.push(&title);
    }

    pub fn stop(&mut self) {
        if let Some(opts) = &self.opts {
            if opts.write_once.is_some_and(|write_once| write_once == true)
                && self.title_list.contains(&self.title)
            {
                return;
            }
        }
        let elapsed_time = self.start_time.elapsed();
        let (elapsed_time, elapsed_time_color) = (
            format!("{:.2?}", elapsed_time),
            select_elapsed_time_color(&elapsed_time),
        );
        if matches!(self.config.log_level, LogLevel::Performance) {
            println!(
                "[{}] >> {}",
                self.title.color(self.color),
                elapsed_time.color(elapsed_time_color)
            );
        }
    }
}

fn select_elapsed_time_color(e_time: &Duration) -> Color {
    if e_time.as_millis().ge(&1) {
        return Color::BrightRed;
    }

    return Color::BrightGreen;
}
