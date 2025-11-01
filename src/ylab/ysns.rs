pub use super::ytfk::bsu as ybsu;
/// # YSNS
/// provides interfaces to sensors.
pub use super::*;
pub use ytfk::data::Sample as GenericSample;
//type MasterAsyncI2c = i2c::I2c<'static, mcu::mode::Async, mcu::i2c::Master>;

pub mod moi {
    use super::*;
    pub use mcu::gpio::{Input, Pull};
    pub use mcu::peripherals::{PA10, PB3, PB4, PB5}; // D2 .. D5
    pub use mcu::exti::ExtiInput;
    pub type Measure = bool;
    pub type Reading<const N: usize> = [Measure; N];
    pub type Sample<const N: usize> = GenericSample<Measure, N>;

    /* result channel */
    //pub static RESULT: Signal<RawMutex, Sample>  = Signal::new();

    /* control channels */
    pub static READY: AtomicBool = AtomicBool::new(false);
    pub static SAMPLE: AtomicBool = AtomicBool::new(true);

    #[embassy_executor::task]
    pub async fn task(
        mut moi_0: ExtiInput<'static>, // PA10
        mut moi_1: ExtiInput<'static>, // PB3
        sensory: u8,
    ) {
        //pub async fn task(pins: [AnyPin; 4], trigger: [(bool, Option<bool>); 4], hz: u64, sensory: u8) {
        println!("Starting MOI task");
        use embassy_futures::select::select;
        loop {
            if SAMPLE.load(ORD) {
                println!("MOI: await");
                select(moi_0.wait_for_any_edge(), moi_1.wait_for_any_edge()).await;
                println!("Event detected");
                let reading = [moi_0.get_level().into(), moi_1.get_level().into()];
                let sample = Sample {
                    sensory: sensory,
                    time: Instant::now(),
                    read: reading,
                };

                ybsu::SINK.send(sample.into()).await;
            };
        }
    }
}

/*pub mod yco2 {
    use super::*;
    //use mcu::peripherals::I2C1 as ThisI2C;
    use scd4x;

    /* control channels */
    pub static READY: AtomicBool = AtomicBool::new(false);
    pub static SAMPLE: AtomicBool = AtomicBool::new(true);

    // Generic result
    const N: usize = 3;
    pub type Measure = f32;
    pub type Reading = [Measure; N];
    /// <--- 4 channel is total accel for now
    pub type Sample = GenericSample<Measure, N>;

    //#[embassy_executor::task]

    use ehal::adapter::{BlockingAsync, YieldingAsync};
    pub async fn inner_task<I>(i2c_bus: &'static SharedI2cBus, hz: u64, sensory: u8)
    where
        I: mcu::i2c::Instance,
    {
        let i2c = SharedI2cDevice::new(&i2c_bus);
        let i2c = YieldingAsync::new(i2c);
        let mut sensor = scd4x::Scd4x::new(i2c.into(), time::Delay); // <-- this makes it sybc or async
                                                              //sensor.wake_up(); <---- This fails
        println!("Starting up SCD41");
        match sensor.stop_periodic_measurement() {
            Ok(_) => {}
            Err(_) => {
                println!("Stopping periodic measurements failed.")
            }
        }

        match sensor.reinit() {
            Ok(_) => {
                READY.store(true, ORD);
            }
            Err(_) => {
                println!("SCD41 reinit failed.")
            }
        }

        let mut ticker = Ticker::every(Duration::from_secs(5));
        let mut sample: Sample;

        loop {
            if SAMPLE.load(ORD) {
                println!("SCD41 active");
                match sensor.measurement() {
                    Err(_) => {
                        println!("SCD41 single shot failed");
                    }
                    Ok(_) => {
                        println!("SCD41 read");
                        ticker.next().await;
                        match sensor.measurement() {
                            Err(_) => {
                                println!("SCD41 read failed.");
                            }
                            Ok(raw) => {
                                let reading: Reading =
                                    [raw.co2 as f32, raw.humidity as f32, raw.temperature as f32];
                                sample = Sample {
                                    sensory: sensory,
                                    time: Instant::now(),
                                    read: reading,
                                };
                                ybsu::SINK.send(sample.into()).await;
                            }
                        };
                    }
                };
            };
        }
    }
}*/

pub mod adc {
    /// STM32
    pub use super::{mcu, ytfk::bsu as ybsu, Channel, Mutex, Ordering};
    pub use super::*;
    use mcu::peripherals::{ADC1, PA0, PA1, PA4, PB0, PC0, PC1, PC2, PC3};
    //use mcu::peripherals::{ADC3, PF3, PF4, PF5, PF6, PF7, PF8, PF9, PF10};
    use mcu::adc::{Adc, SampleTime};
    ///
    const N: usize = 8;
    pub type Measure = u16;
    pub type Reading = [Measure; N];
    pub type Sample = GenericSample<Measure, N>;

    /// Static channels for status and data
    ///
    /// ADC banks will use this to indicate ready-to-poll
    /// and send the data.
    pub static READY: AtomicBool = AtomicBool::new(false);
    pub static SAMPLE: AtomicBool = AtomicBool::new(true);

    //type AdcPin: embedded_hal::adc::Channel<mcu::adc::Adc<'static>> + mcu::gpio::Pin;

    /// Task for ADC controller 1 with eight pins
    ///

    #[embassy_executor::task]
    pub async fn adcbank_1(
        // STM32
        mut adc: Adc<'static, ADC1>,
        mut pins: ( Peri<'static,PA0>, Peri<'static,PA1>, Peri<'static,PA4>, Peri<'static,PB0>,
                    Peri<'static,PC1>, Peri<'static,PC0>, Peri<'static,PC3>, Peri<'static,PC2>),
        //
        hz: u64,
        sensory: u8,
    ) {
        println!("Starting ADC task");
        //let state: Atomic<super::State> = Atomic::new(State::Offline);
        let mut ticker = Ticker::every(Duration::from_hz(hz));
        let mut _vrefint = adc.enable_vrefint();

        let mut sample: Sample;
        adc.set_sample_time(SampleTime::CYCLES3);
        adc.set_resolution(mcu::adc::Resolution::BITS12);
        //println!("ADC set");
        loop {
            if SAMPLE.load(ORD) {
                let reading = [

                    adc.blocking_read(&mut pins.1),
                    adc.blocking_read(&mut pins.1),
                    adc.blocking_read(&mut pins.2),
                    adc.blocking_read(&mut pins.3),
                    adc.blocking_read(&mut pins.4),
                    adc.blocking_read(&mut pins.5),
                    adc.blocking_read(&mut pins.6),
                    adc.blocking_read(&mut pins.7),
                ];
                sample = Sample {
                    sensory: sensory,
                    time: Instant::now(),
                    read: reading,
                };
                ybsu::SINK.send(sample.into()).await;
            };
            ticker.next().await;
        }
    }
}


pub mod sen_five {
    use super::*;
    const N: usize = 8;
    type Measure = f32;
    type Reading = [Measure; N];
    pub type Sample = GenericSample<Measure, N>;

    // control channels
    pub static READY: AtomicBool = AtomicBool::new(false);
    pub static SAMPLE: AtomicBool = AtomicBool::new(true);

    use embedded_hal::delay::DelayNs;
    use embedded_hal::i2c::I2c;
    use sen5x::Sen5x;
    use sen5x_rs as sen5x;

    pub struct Sensor<I, D>
    where
        I: I2c,
        D: DelayNs,
    {
        sensor: Sen5x<I, D>,
        pub id: u8,
        pub interval: Duration,
    }

    impl<I, D> Sensor<I, D>
    where
        I: I2c,
        D: DelayNs,
    {
        pub fn new(i2c: I, delay: D, id: u8, interval: Duration) -> Self {
            Self {
                sensor: Sen5x::new(i2c, delay),
                id: id,
                interval: interval,
            }
        }

        pub fn set_interval(&mut self, interval: Duration) {
            self.interval = interval;
        }

        pub fn set_hz(&mut self, hz: u32) {
            self.interval = Duration::from_hz(hz.into());
            todo!()
        }

        pub fn init(&mut self) -> Result<(), ()> {
            match self.sensor.reinit() {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            }
        }

        pub fn read(&mut self) -> Result<Reading, ()> {
            let reading = self.sensor.measurement();
            match reading {
                Ok(r) => Ok([
                    r.humidity,
                    r.nox_index,
                    r.pm1_0,
                    r.pm2_5,
                    r.pm4_0,
                    r.pm10_0,
                    r.temperature,
                    r.voc_index,
                ]),
                _ => Err(()),
            }
        }

        pub fn sample(&mut self) -> Result<Sample, ()> {
            let reading = self.read();
            match reading {
                Ok(reading) => Ok(Sample {
                    sensory: self.id,
                    time: Instant::now(),
                    read: reading,
                }),
                Err(_) => Err(()),
            }
        }
    }

    //use mcu::peripherals::I2C1 as ThisI2C;

    #[embassy_executor::task]
    pub async fn task(i2c: MasterAsyncI2c, interval: Duration, sensory: u8) {
        let mut sensor = Sensor::new(i2c, time::Delay, sensory, interval);
        match sensor.init() {
            Err(_) => {
                println!("Sensor setup failed");
                return;
            } // connection error => end task
            Ok(_) => {}
        }

        let mut ticker = Ticker::every(interval);
        READY.store(true, ORD);
        println!("Sen5 ready");

        loop {
            if SAMPLE.load(ORD) {
                match sensor.sample() {
                    Ok(sample) => {
                        ybsu::SINK.send(sample.into()).await;
                    }
                    Err(_) => {}
                }
            };
            ticker.next().await;
        }
    }
}
