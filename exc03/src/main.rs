mod view;
mod model;

use model::ID;
use model::{Engine, SteeringWheel, Accelerometer, Tires};
use model::SensorInterface;
use view::{Level, Logger};


fn main() {
    // declaration of every sensor in the vehicle
    let mut steering_wheel = SteeringWheel { id: ID(0), wheel_angle: 0. };
    let mut engine = Engine { id: ID(1), rpm: 2500, temperature: 81 };
    let mut accelerometer = Accelerometer {id: ID(2), vehicle_speed: 15., vehicle_acceleration: (-5, 2, -1)};
    let mut tires = Tires {id: ID(3), pressure: (2.15, 2.15, 2.15, 2.16)};

    // create logger and add sensors to "watch" list
    Logger::new()
        .sampling_freq(1)
        .watch_segment(Box::new(engine), Level::CRITICAL)
        .watch_segment(Box::new(steering_wheel), Level::IMPORTANT)
        .watch_segment(Box::new(accelerometer), Level::INFO)
        .watch_segment(Box::new(tires), Level::INFO)
        .init();
}
