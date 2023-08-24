//! USB Logging
//!
//! This creates the possibility to send log::info/warn/error/debug! to USB serial port.
//! Can be used to improvise data streaming via USB 

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use {defmt_rtt as _, panic_probe as _};
//use core::fmt::Write;
use defmt::*;
use embassy_executor::Spawner;

use embassy_time::{Instant, Duration, Ticker};

use embassy_stm32 as hal;

use hal::dma::NoDma;
use hal::usart::{Config, Uart};
use hal::{bind_interrupts, peripherals, usart};
//use heapless::String;
use {defmt_rtt as _, panic_probe as _};

type Measure = (Instant, [u32; 1]);
use embassy_sync::channel::Channel;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
pub static SINK: Channel<CriticalSectionRawMutex, Measure, 2> = Channel::new();
use itoa;

#[embassy_executor::task]
pub async fn bsu_task(mut usart: Uart<'static, peripherals::USART3, peripherals::DMA1_CH3>) {
    let mut i2a = itoa::Buffer::new();
    loop {
        let measure = SINK.recv().await;
        let reading = measure.1[0];
        let encoded = reading.to_ne_bytes();
        //let encoded = bincode::serialize(&measure).unwrap();
        // let encoded = i2a.format(measure.1[0]).as_bytes();
        usart.write(&encoded).await.unwrap();
        }
    }


/* MAIN */
#[embassy_executor::main]
async fn init(spawner: Spawner) {
    let p = hal::init(Default::default());
    
    bind_interrupts!(struct Irqs {
        USART3 => usart::InterruptHandler<peripherals::USART3>;
    });
    let usart = Uart::new(p.USART3, p.PD9, p.PD8, Irqs, p.DMA1_CH3, NoDma, Config::default());
    spawner.spawn(bsu_task(usart)).unwrap();

    let mut counter = 0;
    let mut ticker = Ticker::every(Duration::from_millis(500));
    loop {
        ticker.next().await;
        counter += 1;
        SINK.send((Instant::now(), [counter])).await;
    }
}
