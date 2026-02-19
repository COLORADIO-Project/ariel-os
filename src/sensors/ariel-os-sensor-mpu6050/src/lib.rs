//! Driver for the InvenSense [MPU6050] accelerometer and gyroscope.
//!
//! Compatible with [`ariel_os_sensors::Sensor`].
//!
//! [MPU6050]: https://invensense.tdk.com/wp-content/uploads/2015/02/MPU-6000-Datasheet1.pdf
//!
#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

pub mod i2c;

const PART_NUMBER: &str = "MPU6050";

const ACC_SCALING: i8 = -3;
const GYR_SCALING: i8 = -3;
const TEMP_SCALING: i8 = 0;

#[expect(dead_code)]
const DEVICE_ID: u8 = 0xa0;
