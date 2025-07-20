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

use ev3dev_lang_rust::{motors, sensors, Ev3Error};
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
    distance: i32,
}

pub struct Peripherals {
    pub drive: motors::LargeMotor,
    pub gyroscope: sensors::GyroSensor,
    pub ultrasonic: sensors::UltrasonicSensor,
}

impl RobotState {
    pub fn stop(
        &mut self,
        peripherals: &mut Peripherals,
    ) -> Result<(), ev3dev_lang_rust::Ev3Error> {
        info!("stop");
        self.move_state = MoveState::Stop;
        peripherals.drive.set_duty_cycle_sp(0)
    }

    pub fn forwards(
        &mut self,
        peripherals: &mut Peripherals,
    ) -> Result<(), ev3dev_lang_rust::Ev3Error> {
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
        Ok(())
    }
}
