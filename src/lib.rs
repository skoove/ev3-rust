use std::{thread::sleep, time::Duration};

use log::info;

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum MoveState {
    #[default]
    Stop,
    Forwards,
    Backwards,
}

#[derive(Default, Clone, Copy)]
pub struct RobotState {
    move_state: MoveState,
    sensor_data: SensorData,
}

struct SensorData {
    angle: i32,
    touching_something: bool,
    distance: i32,
}

pub struct Peripherals {
    pub drive: ev3dev_lang_rust::motors::LargeMotor,
    pub gyro: ev3dev_lang_rust::sensors::GyroSensor,
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
        info!("forwards");

        if self.move_state == MoveState::Stop {
            self.move_state = MoveState::Forwards;
            peripherals.drive.set_duty_cycle_sp(100)?
        } else {
            self.move_state = MoveState::Forwards;
            peripherals.drive.set_duty_cycle_sp(100)?;
            sleep(Duration::from_millis(500));
        }

        Ok(())
    }

    pub fn backwards(
        &mut self,
        peripherals: &mut Peripherals,
    ) -> Result<(), ev3dev_lang_rust::Ev3Error> {
        info!("backwards");

        if self.move_state == MoveState::Stop {
            self.move_state = MoveState::Backwards;
            peripherals.drive.set_duty_cycle_sp(-100)?
        } else {
            self.move_state = MoveState::Backwards;
            peripherals.drive.set_duty_cycle_sp(-100)?;
            sleep(Duration::from_millis(500));
        }

        Ok(())
    }
}
