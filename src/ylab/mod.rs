#![no_std]
#![feature(type_alias_impl_trait)]

/// # YLab Edge Pro
/// provides methods to run *banks of sensors (BOS)* concurrently.
/// 
/// + STM32 (Nucleo) boards
pub use embassy_stm32 as hal;
/// + asynchronous data exchange
pub use embassy_sync::channel::Channel;
pub use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Mutex;
pub use core::sync::atomic::Ordering as Ordering;
/// + common classes for samples from BOS:
use heapless::String;
use core::fmt::Write;
use serde::{Serialize, Deserialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct Sample {
    pub dev: i8,
    pub time: i32,
    pub read: [u16;4],
}
/// + methods to convert to CSV
impl Sample {
    pub fn to_csv(&self) -> String<32>{
        let mut line: String<32> = String::new();
        core::write!(&mut line,
                        "{},{},{},{},{},{}\n",
                        self.time, self.dev, 
                        self.read[0], self.read[1], 
                        self.read[2], self.read[3],).unwrap();
        return line
    }
}
/// ## Sub modules
/// 
/// + sensor banks
pub mod ysns;
/// + data transfer
pub mod ytf;
// /// + (limited) UI support 
// pub mod yuio; // YLab UI Output
// pub mod yuii; // YLab UI Input
