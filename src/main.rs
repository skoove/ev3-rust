use std::{thread::sleep, time::Duration};

use ev3dev_lang_rust::{sound, Ev3Error};

fn main() -> Result<(), Ev3Error> {
    sound::play("weezer.wav")?.wait();

    sleep(Duration::from_millis(250));

    sound::speak("i wish for death")?.wait();

    sleep(Duration::from_millis(500));

    sound::speak("kill me");

    Ok(())
}
