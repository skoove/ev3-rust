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

use ev3dev_lang_rust::{motors, sensors, Ev3Error, Led};
use log::info;
use std::{thread::sleep, time::Duration};

#[derive(Default, PartialEq, Eq, Clone, Copy, Debug)]
pub enum MoveState {
    #[default]
    Stop,
    Forwards,
    Backwards,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct RobotState {
    move_state: MoveState,
    sensor_data: SensorData,
}

#[derive(Default, Clone, Copy, Debug)]
struct SensorData {
    angle: i32,
    distance: f32,
}

pub struct Peripherals {
    pub drive: motors::LargeMotor,
    pub gyroscope: sensors::GyroSensor,
    pub ultrasonic: sensors::UltrasonicSensor,
    pub led: Led,
}

impl Peripherals {
    /// Calibartes gyroscope
    /// Sets led to red, sets the gyro to cal mode then sleeps for a second,
    /// then turns LED green again and reutrns Ok(())
    pub fn calibrate_gyroscope(&mut self) -> Result<(), Ev3Error> {
        self.led.set_color(Led::COLOR_RED)?;
        info!("gyroscope calibration begin");

        self.gyroscope.set_mode_gyro_cal()?;
        sleep(Duration::from_secs(1));
        self.gyroscope.set_mode_gyro_ang()?;

        self.led.set_color(Led::COLOR_GREEN)?;
        info!("gyroscope calibration done");
        Ok(())
    }
}

impl RobotState {
    pub fn stop(
        &mut self,
        peripherals: &mut Peripherals,
    ) -> Result<(), ev3dev_lang_rust::Ev3Error> {
        if self.move_state == MoveState::Stop {
            return Ok(());
        }

        info!("stop");
        self.move_state = MoveState::Stop;
        peripherals.drive.set_duty_cycle_sp(0)
    }

    pub fn forwards(
        &mut self,
        peripherals: &mut Peripherals,
    ) -> Result<(), ev3dev_lang_rust::Ev3Error> {
        if self.move_state == MoveState::Forwards {
            return Ok(());
        }

        info!("moving forwards");
        if self.move_state == MoveState::Stop {
            self.move_state = MoveState::Forwards;
            peripherals.drive.set_duty_cycle_sp(100)?
        } else {
            info!("tried to go forwards while moving, stopping first");
            sleep(Duration::from_millis(500));
            self.stop(peripherals)?;
            self.move_state = MoveState::Forwards;
            peripherals.drive.set_duty_cycle_sp(-100)?;
        }

        Ok(())
    }

    pub fn backwards(
        &mut self,
        peripherals: &mut Peripherals,
    ) -> Result<(), ev3dev_lang_rust::Ev3Error> {
        info!("moving backwards");
        if self.move_state == MoveState::Backwards {
            return Ok(());
        }

        if self.move_state == MoveState::Stop {
            self.move_state = MoveState::Backwards;
            peripherals.drive.set_duty_cycle_sp(100)?
        } else {
            info!("tried to go forwards while moving, stopping first");
            sleep(Duration::from_millis(500));
            self.stop(peripherals)?;
            self.move_state = MoveState::Backwards;
            peripherals.drive.set_duty_cycle_sp(-100)?;
        }

        Ok(())
    }

    pub fn update_sensor_data(&mut self, peripherals: &Peripherals) -> Result<(), Ev3Error> {
        self.sensor_data.angle = peripherals.gyroscope.get_angle()?;
        self.sensor_data.distance = peripherals.ultrasonic.get_distance_centimeters()?;
        Ok(())
    }

    pub fn setup(&mut self, peripherals: &mut Peripherals) -> Result<(), Ev3Error> {
        peripherals.drive.run_direct()?;
        peripherals.ultrasonic.set_mode_us_dist_cm()?;
        Ok(())
    }
}
