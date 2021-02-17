use rand_distr::{Distribution, Weibull, Normal};
use rand::prelude::*;
use rand::thread_rng;

/// Common trait used in every sensor below
///
pub trait SensorInterface {
    /// Based on current sensor status returns String for logger
    fn read_status(&self) -> String;
    /// If called, it simulates changes in sensor input.
    /// Often some random distribution is used for this purpose.
    fn update(&mut self);
}

/// Simplification of some UUID system
///
pub struct ID(pub u8);

//// Below all sensors are defined and implemented

pub struct SteeringWheel {
    pub id: ID,
    pub wheel_angle: f32,
}

impl SensorInterface for SteeringWheel {
    fn read_status(&self) -> String {
        format!("wheel_angle={}", self.wheel_angle)
    }

    fn update(&mut self) {
        self.wheel_angle = thread_rng().sample(Weibull::new(10., 0.5).unwrap());
    }
}


pub struct Engine {
    pub id: ID,
    pub rpm: u16,
    pub temperature: i32,
}

impl SensorInterface for Engine {
    fn read_status(&self) -> String {
        format!("engine: rpm={} temp={}", self.rpm, self.temperature).to_string()
    }

    fn update(&mut self) {
        let normal = Normal::new(0.0, 10.0).unwrap();
        let v = normal.sample(&mut rand::thread_rng());
        self.rpm = (self.rpm as i32 + v as i32) as u16;
        self.temperature += v as i32;
    }
}


pub struct Accelerometer {
    pub id: ID,
    pub vehicle_speed: f32,
    pub vehicle_acceleration: (i16, i16, i16),
}

impl SensorInterface for Accelerometer {
    fn read_status(&self) -> String {
        format!("vehicle_acceleration={:?}; vehicle_speed={}", self.vehicle_acceleration, self.vehicle_speed)
    }

    fn update(&mut self) {
        let normal = Normal::new(0.0, 2.0).unwrap();
        self.vehicle_acceleration.0 += normal.sample(&mut rand::thread_rng()) as i16;
        self.vehicle_acceleration.1 += normal.sample(&mut rand::thread_rng()) as i16;
        self.vehicle_acceleration.2 += normal.sample(&mut rand::thread_rng()) as i16;

        self.vehicle_speed += normal.sample(&mut rand::thread_rng())
    }
}


pub struct Tires {
    pub id: ID,
    pub pressure: (f32, f32, f32, f32),
}

impl SensorInterface for Tires {
    fn read_status(&self) -> String {
        format!("tires_pressure={:?}", self.pressure)
    }

    fn update(&mut self) {}
}