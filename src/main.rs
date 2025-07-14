use std::{thread::sleep, time::Duration};

use ev3_rust::{Peripherals, RobotState};
use ev3dev_lang_rust::{
    motors::{LargeMotor, MotorPort},
    Ev3Error,
};

fn main() -> Result<(), Ev3Error> {
    let mut peripherals = Peripherals {
        drive: LargeMotor::get(MotorPort::OutA)
            .expect("failed to find main drive motor, check cables"),
    };

    let mut robot = RobotState::default();

    peripherals.drive.run_direct()?;

    'running: loop {
        robot.forwards(&mut peripherals)?;
        sleep(Duration::from_secs(2));
        robot.backwards(&mut peripherals)?;
        sleep(Duration::from_secs(2));
        robot.stop(&mut peripherals)?;
        sleep(Duration::from_secs(2));
    }
}
