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
use heapless::{String,Vec};
//use postcard;
use serde::{Serialize, Deserialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct Sample {
    pub dev: i8,
    pub time: i32,
    pub read: [u16;8],
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
