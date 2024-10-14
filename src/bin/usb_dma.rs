#![no_std]
#![no_main]

use core::fmt::Write;

use defmt::*;
use embassy_executor::Spawner;
//use embassy_stm32::usart::{Config, Uart};
//use embassy_stm32::{bind_interrupts, peripherals, usart};
use ylab::hal;
use hal::bind_interrupts;
use hal::peripherals::USB_OTG_FS;
use hal::usb_otg::{Driver, InterruptHandler, Config};
use embassy_usb_logger::*;
use log::LevelFilter;
use heapless::String;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    OTG_FS => InterruptHandler<USB_OTG_FS>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let config = Config::default();
    //let mut usart = Uart::new(p.USART3, p.PC11, p.PC10, Irqs, p.DMA1_CH3, p.DMA1_CH1, config).unwrap();
    let mut usart = Uart::new(p.USART1, p.PA10, p.PA9, Irqs, p.DMA1_CH3, p.DMA1_CH1, config).unwrap();

    for n in 0u32.. {
        //println!("Hello DMA World {}!\r\n", n);
        let mut s: String<128> = String::new();
        core::write!(&mut s, "Hello DMA World {}!\r\n", n).unwrap();
        unwrap!(usart.write(s.as_bytes()).await);
        //info!("wrote DMA");
    }
}