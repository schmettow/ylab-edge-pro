#![no_std]
#![feature(type_alias_impl_trait)]

use heapless::String;
use core::fmt::Write;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Sample {
    pub dev: i8,
    pub time: i32,
    pub read: [u16;4],
}

impl Sample {
    pub fn to_csv(&self) -> String<128>{
        let mut line: String<128> = String::new();
        core::write!(&mut line,
                        "{},{},{},{},{},{}\n",
                        self.dev, self.time,
                        self.read[0], self.read[1], 
                        self.read[2], self.read[3],).unwrap();
        return line
    }
}

pub use embassy_stm32 as hal;


pub mod ysns; // Ylab sensors
// pub mod yuio; // YLab UI Output
// pub mod yuii; // YLab UI Input
//pub mod ytfk; // YLab transfer formats
pub mod ytf;