/// # YSNS 
/// provides interfaces to sensors.

pub mod adc {
    pub use super::super::{hal, Sample, Channel, Mutex, ytf::bsu as ybsu, Ordering};
    use embassy_time::{Duration, Ticker, Instant};
    use hal::peripherals::{ADC1, PA3, PC0, PC3, PC1};
    use hal::peripherals::{ADC3, PF3, PF5, PF10, PF8};
    //use hal::adc::{config::AdcConfig, config::SampleTime, Adc};
    use hal::adc::{Adc, SampleTime};

    /* data */
    pub type Reading = [u16; 4];
    
    // pub type Sample = (Instant, Reading);

    
    /* control channels */
    use core::sync::atomic::AtomicBool;
    pub static READY: AtomicBool = AtomicBool::new(false);
    pub static SAMPLE: AtomicBool = AtomicBool::new(false);
    
    //type AdcPin: embedded_hal::adc::Channel<hal::adc::Adc<'static>> + hal::gpio::Pin;

    #[embassy_executor::task]
    pub async fn task_0(mut adc: Adc<'static, ADC1>,
                    mut pins: (PA3, PC0, PC3, PC1),
                    hz: u64) {
        //let state: Atomic<super::State> = Atomic::new(State::Offline);
        let mut ticker = Ticker::every(Duration::from_hz(hz));
        let mut vrefint = adc.enable_vrefint();
        let mut reading: Reading;
        let mut sample: Sample;
        adc.set_sample_time(SampleTime::Cycles3);
        adc.set_resolution(hal::adc::Resolution::TwelveBit);
        loop {
            ticker.next().await;
            if SAMPLE.load(Ordering::Relaxed){
                    reading =  [adc.read(&mut pins.0), 
                                adc.read(&mut pins.1),
                                adc.read(&mut pins.2),
                                adc.read(&mut pins.3),];
                    sample = Sample{dev: 0, 
                                    time: Instant::now().as_millis() as i32, 
                                    read: reading};
                    ybsu::SINK.send(sample).await;
                }
            }                
        }

    #[embassy_executor::task]
    pub async fn task_1(mut adc: Adc<'static, ADC3>,
                    mut pins: (PF3, PF5, PF10, PF8),
                    hz: u64) {
        //let state: Atomic<super::State> = Atomic::new(State::Offline);
        const DEV: i8 = 1;
        let mut ticker = Ticker::every(Duration::from_hz(hz));
        let mut reading: Reading;
        let mut sample: Sample; 
        adc.set_sample_time(SampleTime::Cycles3);
        adc.set_resolution(hal::adc::Resolution::TwelveBit);
        loop {
            ticker.next().await;
            if SAMPLE.load(Ordering::Relaxed){
                    reading =  [adc.read(&mut pins.0),
                                adc.read(&mut pins.1),
                                adc.read(&mut pins.2),
                                adc.read(&mut pins.3),];
                    //reading = [2,9,1,2];
                    sample = Sample{dev: DEV, 
                                    time: Instant::now().as_millis() as i32, 
                                    read: reading};
                    ybsu::SINK.send(sample).await;
                }
            }                
        }
    }

