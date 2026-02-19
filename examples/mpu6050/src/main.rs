#![no_std]
#![no_main]

mod i2c_bus;
mod pins;
mod sensors;

use ariel_os::{
    debug::log::{error, info},
    sensors::REGISTRY,
    time::Timer,
};
use ariel_os_sensors::{Category, Label, Reading};

use crate::pins::Peripherals;

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: Peripherals) {
    i2c_bus::init(peripherals.i2c);
    sensors::init().await;

    loop {
        let Some(sensor) = REGISTRY
            .sensors()
            .find(|s| s.categories().contains(&Category::AccelerometerGyroscope))
        else {
            info!("There aren't any registered temperature sensors");
            break;
        };

        let Some(label) = sensor.label() else {
            info!("Sensor has no label");
            break;
        };
        info!("Found sensor with label: {}", label);

        if let Err(err) = sensor.trigger_measurement() {
            error!("Error when triggering a measurement: {}", err);
            Timer::after_secs(2).await;
            continue;
        }
        let reading = sensor.wait_for_reading().await;

        match reading {
            Ok(samples) => {
                for (reading_channel, sample) in samples.samples() {
                    let value = sample.value().unwrap();
                    let scaled_value = scaled_to_f32(value, reading_channel.scaling());
                    info!(
                        "{}: {} {}",
                        reading_channel.label(),
                        scaled_value,
                        reading_channel.unit()
                    );
                }
            }
            Err(err) => {
                error!("Error when reading: {}", err);
            }
        }
        Timer::after_secs(2).await;
    }
}

pub fn scaled_to_f32(value_f32: i32, scale10: i8) -> f32 {
    let mut factor = 1.0_f32;

    if scale10 >= 0 {
        for _ in 0..(scale10 as u32) {
            factor *= 10.0;
        }
        (value_f32 as f32) * factor
    } else {
        for _ in 0..((-scale10) as u32) {
            factor *= 10.0;
        }
        (value_f32 as f32) / factor
    }
}
