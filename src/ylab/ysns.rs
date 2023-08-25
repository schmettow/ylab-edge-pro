pub use super::Sample;
pub use super::hal;
pub use crate::ytf::bsu as ybsu;

use atomic_enum::atomic_enum;
#[atomic_enum]
#[derive(PartialEq)]
pub enum SensorState {Offline, Ready, Record}

pub mod adc {
    use super::Sample;
    use super::hal;
    use super::ybsu;
    use super::SensorState;
    use embassy_time::{Duration, Ticker, Instant};
    use hal::peripherals::{ADC1, PA3, PC0, PC3, PC1};
    use hal::adc::Adc;

    /* data */
    
    pub type Reading = [u16; 4];
    
    // pub type Sample = (Instant, Reading);

    
    /* control channels */
    pub use core::sync::atomic::Ordering;
    use core::sync::atomic::AtomicBool;
    pub static READY: AtomicBool = AtomicBool::new(false);
    pub static RECORD: AtomicBool = AtomicBool::new(false);
    
    //type AdcPin: embedded_hal::adc::Channel<hal::adc::Adc<'static>> + hal::gpio::Pin;
    static State: SensorState = SensorState::Offline;
    
    #[embassy_executor::task]
    pub async fn task(mut adc: Adc<'static, ADC1>,
                    mut pins: (PA3, PC0, PC3, PC1),
                    hz: u64) {
        let mut ticker = Ticker::every(Duration::from_hz(hz));
        let mut vrefint = adc.enable_vrefint();
        let mut reading: Reading;
        let mut sample: Sample; 
        loop {
            ticker.next().await;
            //match State.load(Ordering::Relaxed) {
            if RECORD.load(Ordering::Relaxed){
                let _vrefint_sample = adc.read_internal(&mut vrefint);
                reading =  [adc.read(&mut pins.0), 
                            adc.read(&mut pins.1),
                            adc.read(&mut pins.2),
                            adc.read(&mut pins.3),];
                sample = Sample{dev: 0, time: Instant::now().as_millis() as i32, 
                    read: reading};
                
                ybsu::SINK.send(sample).await;
                };
            }                
        }
    }

