// ev3-rust
// Copyright (C) 2025 skoove

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::{thread::sleep, time::Duration};

use env_logger::Env;
use ev3_rust::{Peripherals, RobotState};
use ev3dev_lang_rust::{
    motors::{LargeMotor, MotorPort},
    sensors::{GyroSensor, UltrasonicSensor},
    Ev3Error, Led,
};
use log::info;

fn main() -> Result<(), Ev3Error> {
    env_logger::init_from_env(Env::new().default_filter_or("trace"));

    info!("hello!");

    let mut peripherals = Peripherals {
        drive: LargeMotor::find()
            .expect("failed to find main drive motor, or there is more than one"),
        gyroscope: GyroSensor::find().expect("failed to find gyroscope, or there is more than one"),
        ultrasonic: UltrasonicSensor::find()
            .expect("failed to find ultrasonic sensor, or there is more than one"),
    };

    let mut robot = RobotState::default();
    let led = Led::new()?;

    peripherals.drive.run_direct()?;

    // gyro cal (i think)
    led.set_color(Led::COLOR_RED)?;
    info!("gyroscope calibration");
    peripherals.gyroscope.set_mode_gyro_cal()?;
    sleep(Duration::from_secs(1));
    peripherals.gyroscope.set_mode_gyro_ang()?;
    led.set_color(Led::COLOR_GREEN)?;
    info!("gyroscope calibration done");

    robot.forwards(&mut peripherals)?;

    loop {
        robot.update_sensor_data(&peripherals)?;
        dbg!(robot);
        sleep(Duration::from_millis(500));
    }
}
