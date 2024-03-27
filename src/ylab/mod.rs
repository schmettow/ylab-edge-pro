#![no_std]

pub use embassy_stm32 as hal;
pub use hal::exti::ExtiInput;
pub use embassy_time as time;
pub use time::{Duration, Ticker, Timer, Instant, Delay};
pub use heapless::{Vec, String};
pub use embassy_sync::mutex::Mutex as Mutex;
pub use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as RawMutex;
pub use embassy_sync::signal::Signal;
pub use embassy_sync::channel::Channel;

pub use core::sync::atomic::Ordering;
pub static RLX: Ordering = Ordering::Relaxed;
pub use core::sync::atomic::AtomicBool;

pub use defmt::println;


/*use core::fmt;
use fmt::Display;
use fmt::Write;*/


pub mod ysns; // Ylab sensors
//pub mod yuio; // YLab UI Output
//pub mod yuii; // YLab UI Input
pub mod ytfk; // YLab transfer formats & kodices

/*
#[derive(Debug,Eq, PartialEq)]
pub struct SensorResult<R> {
    pub time: Instant,
    pub reading: R,
}
 */

#[derive(Debug,Eq, PartialEq, Clone)]
pub struct Sample<T> {
    pub sensory: u8,
    pub time: Instant,
    pub read: T,
}


