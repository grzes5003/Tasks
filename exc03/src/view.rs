use std::{thread, time};

use crate::model::SensorInterface;

/// Defines importance of particular sensor
///
/// sensors with 'CRITICAL' level will
/// be logged more often than 'IMPORTANT' one.
///
/// CRITICAL > IMPORTANT > INFO
///
#[derive(PartialOrd, PartialEq, Ord, Eq)]
pub enum Level {
    CRITICAL = 3,
    IMPORTANT = 2,
    INFO = 1,
}

/// Custom implementation of logging mechanism
///
/// It stores `v_sensors` list of every watched sensor
/// and logs its status periodically.
///
/// # Variables
///
///     `v_sensors`: list of every watched sensor
///     `timer`: clock measuring time since start
///     `fs`: frequency of sampling
///
pub struct Logger {
    v_sensors: Vec<(Level, Box<dyn SensorInterface>)>,
    timer: time::SystemTime,
    fs: time::Duration,
}

impl Logger {

    /// Default constructor
    ///
    pub fn new() -> Logger {
        Logger {
            v_sensors: Vec::new(),
            timer: time::SystemTime::now(),
            fs: time::Duration::from_secs(1)
        }
    }

    /// Define frequency of sampling
    ///
    /// `lambda` in seconds
    pub fn sampling_freq(&mut self, lambda: u64) -> &mut Logger {
        self.fs = time::Duration::from_secs(lambda);
        self
    }

    /// Add sensor reference to watchlist, with appropriate importance level
    ///
    /// # Args
    ///
    ///     `sensor`: referenced vehicle sensor
    ///     `level`: how important log will be
    ///
    pub fn watch_segment<'a>(&'a mut self, sensor: Box<dyn SensorInterface>, level: Level) -> &'a mut Logger {
        self.v_sensors.push((level, sensor));
        self
    }

    /// Start a infinite loop of printing logs
    ///
    pub fn init(&mut self) {
        // order of logs printed in console
        let queue = vec![Level::CRITICAL, Level::CRITICAL, Level::IMPORTANT, Level::INFO];

        // infinite loop, but iterates over `queue`
        for lvl in queue.iter().cycle() {
            thread::sleep(self.fs);
            self.write_log(lvl);

            // update each sensor, so its data differs over time
            for sensor in &mut self.v_sensors {
                sensor.1.update()
            }
        }
    }

    /// Basic method printing based on current level log
    ///
    /// It iterates over `v_sensors`, filters it (based on level)
    /// and reads each sensor data
    ///
    fn write_log(&self, level: &Level) {
        println!("Time {:<9?} | {}",
                 self.timer.elapsed().unwrap().as_secs_f32(),
                 self.v_sensors
                     .iter()
                     .filter(|(lvl,_)| lvl >= level )
                     .map(|(_, segment)| segment.read_status())
                     .collect::<Vec<String>>()
                     .join(" | ")
        );
    }
}