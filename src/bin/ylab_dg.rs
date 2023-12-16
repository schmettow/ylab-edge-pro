#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

/// CONFIGURATION
/// 
/// Adc
static DEV: [bool; 3] = [true, false, false];
static HZ: [u64; 3] = [600, 600, 30];
const BAUD: u32 = 2_000_000; 

/// # YLab Edge
/// 
/// __YLab Edge Pro__ is a sensor recording firmware for STM32 Nucleo boards.
/// 
/// 
/// ## Dependencies
/// 
/// YLab Edge makes use of the Embassy framework, in particular:
/// 

//use defmt::*;

/// +  multi-threading with async
/// + timing using Embassy time 
// use embassy_time::{Duration, Ticker};
/// + peripherals
use embassy_stm32 as hal;
/// + thread-safe data transfer and control
/// 
/// + fbuilt-in ADC sensors
use ylab::ysns::adc as yadc;
/// + data transport/storage
use ylab::ytf::bsu as ybsu;


/// ## UI task
/// 
/// The ui task only signals the state 

#[derive(Debug,  // used as fmt
    Clone, Copy, // because next_state 
    PartialEq, Eq, )] // testing equality
enum AppState {Send}

/// In a usual multi-threaded app you would write the interaction model
/// in the main task. However, with dual-core the main task is no longer 
/// async. Since all communication channels are static, this really doesn't matter.
/// 
/// The initial state is set and a signal is send to the LED.
/// The event loop waits for button events (long or short press) 
/// and changes the states, accordingly.
/// If an actual state change has occured, the state is signaled to the UI 
/// and initialized if that is necessary. In this case, entering Send 
/// starts the sensor sampling.
/// 
/// From an architectural point of view, this is a nice setup, too. 
/// Basically, we are separating the very different tasks of 
/// peripherals/spawning and ui handling. It would be easy to just plugin a 
/// different ui, by just reqriting this task. For example, another ui
/// could use the RGB led to signal states, or collect commands from a serial console.
///
/// Conclusion so far: If we take the Embassy promise for granted, that async is zero-cost, 
/// the separation of functionality into different tasks reduces dependencies. It introduces 
/// the complexity of signalling.
///
/// ## Init
/// 
/// + Initializing peripherals 
/// + spawning tasks
/// + assigning periphs to tasks

use hal::adc;
/// USB
use embassy_stm32::dma::NoDma;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART2 => usart::InterruptHandler<peripherals::USART2>;
});
use embassy_time::Delay;
use embassy_executor::Spawner;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = hal::init(Default::default());
    let mut config = Config::default();
    config.baudrate = BAUD;
    let usart = Uart::new(p.USART2, p.PA3, p.PA2, Irqs, p.DMA1_CH6, NoDma, config);
    spawner.spawn(ybsu::task(usart)).unwrap();
    spawner.spawn(control_task()).unwrap();


    if DEV[0]{
        let mut delay = Delay;
        let adc1 = adc::Adc::new(p.ADC1, &mut delay);
        spawner.spawn(yadc::adcbank_1(adc1, 
                                    (p.PA0, p.PA1, p.PA4, p.PB0, p.PC1, p.PC0, p.PC3, p.PC2), 
                                    HZ[0])).unwrap();
    };
}

/// ## Control task
/// 
/// + capturing button presses
/// + sending commands to the other tasks
/// + giving signals via LED
/// + put text on a display
/// 
/// 

pub use core::sync::atomic::Ordering;

#[embassy_executor::task]
async fn control_task() { 
    let _state = AppState::Send;
    yadc::SAMPLE.store(true, Ordering::Relaxed);
}
        

