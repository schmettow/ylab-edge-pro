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

pub use defmt::println;
pub use core::sync::atomic::Ordering;
pub use core::sync::atomic::AtomicBool;
pub static ORD: Ordering = Ordering::Relaxed; // no parallel computing

pub mod ysns; // Ylab sensors
pub mod ytfk; // YLab transfer formats & kodices


/// Ylab reads sensor data into fixed arrays [Option<f64>; 8]
/// 
/// TODO: Check if this can be reduced to f32 to increase performance
pub const YTF_LEN: usize = 8;
pub type YtfType = f64;
pub type YtfRead = [Option<YtfType>; YTF_LEN];

/// A YTF record is an array of N measures 
/// 
/// with a time-stamp and the id of the sensor array (sensory).
/// Other types than M can be used, if they implement Into<M>.
/// 

pub struct Ytf {
    pub sensory: u8,
    pub time: Instant,
    pub read: YtfRead,
}


/// A Sample is a sensor-typical array of values  
/// with a timestamp and a sensory identifier.
/// 
/// The basic reading type must implement Into<YtfType>.
/// This is supposed to collect readings, where the length depends on the sensor 
/// (e.g. 6 for acceleration sensors)

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Sample<M, const N: usize>
    where M: Into<YtfType>
    {
    pub sensory: u8,
    pub time: Instant,
    pub read: [M; N],
}

/// Converting Sample to YTF
/// 
/// Arrays of arbitrary size are converted to fixed size arrays 
/// with optional types.

impl<M: Into<YtfType>, const N: usize> Into<Ytf> for Sample<M, N> {
    fn into(self) -> Ytf {
        let mut out: YtfRead = [None; YTF_LEN];
        for (i, r) in self.read.into_iter().enumerate() {
            out[i] = Some(r.into());
        }
        Ytf {
            sensory: self.sensory,
            time: self.time,
            read: out
        }
    }
}

/// Display YTF as CSV
/// 
/// the default formatting of YTF is CSV
impl core::fmt::Display for Ytf {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}, {}", self.time.as_micros(), self.sensory).unwrap();
        for r in self.read {
            match r {
                Some(v) => {
                    write!(f, ",{:.5}", v).unwrap();},
                None => {write!(f, ",").unwrap();}
            }
        }
        write!(f, "")
    }
}


