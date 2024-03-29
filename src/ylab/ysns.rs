/// # YSNS 
/// provides interfaces to sensors.

pub use super::*;
pub use super::ytfk::bsu as ybsu;
pub use ytfk::Ytf;
use hal::i2c;

pub mod moi {
    use super::*;
    pub use hal::peripherals::{PA10, PB3, PB5, PB4}; // D2 .. D5
    pub use hal::gpio::{Input, Pull};

    pub type Reading = [bool; 2];
    pub type Sample = crate::Sample<Reading>;
    
    /* result channel */
    //pub static RESULT: Signal<RawMutex, Sample>  = Signal::new();

    /* trigger channel */
    pub static TRIGGER: Channel<RawMutex, Sample, 2> = Channel::new();
    
    /* control channels */
    pub static READY: AtomicBool = AtomicBool::new(false);
    pub static SAMPLE: AtomicBool = AtomicBool::new(true);
    
    #[embassy_executor::task]
    pub async fn task(mut moi_0: ExtiInput<'static, PA10>, mut moi_1: ExtiInput<'static, PB3>, sensory: u8) {
    //pub async fn task(pins: [AnyPin; 4], trigger: [(bool, Option<bool>); 4], hz: u64, sensory: u8) {
        println!("Starting MOI task");
        let mut sample: Sample; 
        let mut reading: Reading;
        use embassy_futures::select::select;
        loop {
            if SAMPLE.load(RLX){
                println!("MOI: await");
                select( moi_0.wait_for_any_edge(), 
                        moi_1.wait_for_any_edge()).await;
                println!("Event detected");
                reading = [moi_0.get_level().into(), moi_1.get_level().into()];
                sample = Sample{
                            sensory: sensory,
                            time: Instant::now(), 
                            read: reading};
                
                ybsu::SINK.send(to_ytf(sample.clone())).await;
                // TRIGGER.send(sample).await; <----- Makes it hang
                };
            }                
        }


        pub fn to_ytf(sample: Sample) -> ytfk::Ytf {
            Ytf { sensory: sample.sensory, 
                    time: sample.time, 
                    read: [ Some(sample.read[0].into()), 
                            Some(sample.read[1].into()),
                            None,None, None, None, None, None] }
        }
    }
   


pub mod yco2 {
    use super::*;
    use hal::peripherals::I2C1 as ThisI2C;
    use scd4x;

    
    /* control channels */
    pub static READY: AtomicBool = AtomicBool::new(false);
    pub static SAMPLE: AtomicBool = AtomicBool::new(true);

    // Generic result
    pub type Reading = [f32; 3];
    pub type Sample = crate::Sample<Reading>;

    pub fn to_ytf(sample: Sample) -> ytfk::Ytf {
        Ytf { sensory: sample.sensory, 
                time: sample.time, 
                read: [ Some(sample.read[0].into()), 
                        Some(sample.read[1].into()),
                        Some(sample.read[2].into()),
                        None, None, None, None, None] }
    }
    
    #[embassy_executor::task]
    pub async fn task(  i2c: i2c::I2c<'static, ThisI2C>, sensory: u8) { 
        //DISP.signal([None, None, None, Some("CO2 start".try_into().unwrap())]);        
        let mut sensor = scd4x::Scd4x::new(i2c, time::Delay); // <-- this makes it sybc or async
        //sensor.wake_up(); <---- This fails
        println!("Starting up SCD41");
        match sensor.stop_periodic_measurement() {
            Ok(_) => {},
            Err(_) => {println!("Stopping periodic measurements failed.")}
        }

        match sensor.reinit() {
            Ok(_) => {READY.store(true, RLX);},
            Err(_) => {println!("SCD41 reinit failed.")},
        }
    
        let mut ticker = Ticker::every(Duration::from_secs(5));
        let mut sample: Sample;
        
        loop {
            if SAMPLE.load(RLX){
                println!("SCD41 active");
                match sensor.measure_single_shot_non_blocking() {
                    Err(_) => {println!("SCD41 single shot failed");},
                    Ok(_) => {
                        println!("SCD41 read");
                        ticker.next().await;
                        match sensor.measurement() {
                            Err(_) => {println!("SCD41 read failed.");},
                            Ok(raw) => {
                                let reading: Reading = [raw.co2 as f32, raw.humidity as f32, raw.temperature as f32];
                                sample = Sample{sensory: sensory, time: Instant::now(), read: reading};
                                ybsu::SINK.send(to_ytf(sample)).await;
                            },
                        };
                        
                        }
                    };
                
                };               
            }
        }
    }


pub mod adc {
    pub use super::*;
    pub use super::super::{hal, Channel, Mutex, ytfk::bsu as ybsu, Ordering};
    use hal::peripherals::{ADC1, PA0, PA1, PA4, PB0, PC1, PC0, PC3, PC2};
    //use hal::peripherals::{ADC3, PF3, PF4, PF5, PF6, PF7, PF8, PF9, PF10};
    use hal::adc::{Adc, SampleTime};

    // data
    // pub type Measure = SensorResult<Reading>;
    pub type Reading = [u16; 8];
    pub type Sample = crate::Sample<Reading>;
    
    // control channels
    pub static READY: AtomicBool = AtomicBool::new(false);
    pub static SAMPLE: AtomicBool = AtomicBool::new(true);
    
    pub fn to_ytf(sample: Sample) -> ytfk::Ytf {
        Ytf { sensory: sample.sensory, 
                time: sample.time, 
                read: [ Some(sample.read[0].into()), 
                        Some(sample.read[1].into()),
                        Some(sample.read[2].into()),
                        Some(sample.read[3].into()),
                        Some(sample.read[4].into()),
                        Some(sample.read[5].into()),
                        Some(sample.read[6].into()),
                        Some(sample.read[7].into()),] }
    }
    
    //type AdcPin: embedded_hal::adc::Channel<hal::adc::Adc<'static>> + hal::gpio::Pin;

    #[embassy_executor::task]
    pub async fn adcbank_1(mut adc: Adc<'static, ADC1>,
                    mut pins: (PA0, PA1, PA4, PB0, PC1, PC0, PC3, PC2),
                    hz: u64,
                    sensory: u8) {
        println!("Starting ADC task");
        //let state: Atomic<super::State> = Atomic::new(State::Offline);
        let mut ticker = Ticker::every(Duration::from_hz(hz));
        let mut _vrefint = adc.enable_vrefint();

        let mut sample: Sample;
        adc.set_sample_time(SampleTime::Cycles3);
        adc.set_resolution(hal::adc::Resolution::TwelveBit);
        //println!("ADC set");
        loop {
            if SAMPLE.load(RLX){
                let reading =  
                    [adc.read(&mut pins.0), 
                    adc.read(&mut pins.1),
                    adc.read(&mut pins.2),
                    adc.read(&mut pins.3),
                    adc.read(&mut pins.4), 
                    adc.read(&mut pins.5),
                    adc.read(&mut pins.6),
                    adc.read(&mut pins.7),];
                sample = Sample{sensory: sensory, time: Instant::now(), 
                                read: reading};
                ybsu::SINK.send(to_ytf(sample)).await;
                };
            ticker.next().await;
            }                
        }
    }

pub mod yxz_lsm6 {

    use super::*;
    use hal::peripherals::I2C1 as ThisI2C;
    use lsm6dsox::*;
    use Lsm6dsox as Lsm6;
    use accelerometer::Accelerometer;

    pub type Reading = [f32; 6];
    pub type Sample = crate::Sample<Reading>;
    
    // control channels
    pub static READY: AtomicBool = AtomicBool::new(false);
    pub static SAMPLE: AtomicBool = AtomicBool::new(true);
    
    pub fn to_ytf(sample: Sample) -> ytfk::Ytf {
        Ytf { sensory: sample.sensory, 
                time: sample.time, 
                read: [ Some(sample.read[0].into()), 
                        Some(sample.read[1].into()),
                        Some(sample.read[2].into()),
                        Some(sample.read[3].into()),
                        Some(sample.read[4].into()),
                        Some(sample.read[5].into()),None,None] }
    }

    #[embassy_executor::task]
    pub async fn task(  i2c: i2c::I2c<'static, ThisI2C>,
                        hz: u64,
                        sensory: u8) { 
        let mut sensor 
            = Lsm6::new(i2c, SlaveAddress::Low, time::Delay);
        let success = sensor.setup();
        match success {
            Err(_) => return, // connection error => end task
            Ok(_) => {},
        }

        sensor.set_accel_sample_rate(DataRate::Freq416Hz).unwrap();
        sensor.set_accel_scale(AccelerometerScale::Accel2g).unwrap();
        sensor.set_gyro_sample_rate(DataRate::Freq416Hz).unwrap();
        sensor.set_gyro_scale(GyroscopeScale::Dps250).unwrap();
        let _ = sensor.accel_norm().unwrap() ;
        let _ = sensor.angular_rate().unwrap();
        let mut ticker 
                = Ticker::every(Duration::from_hz(hz));
        let mut reading: Reading;
        let mut sample: Sample;
        READY.store(true, RLX);
        
        //let mut i = 0;
        loop {
            if SAMPLE.load(RLX){
                let accel = sensor.accel_norm().unwrap();
                let gyro = sensor.angular_rate().unwrap();
                reading = [ accel.x, accel.y, accel.z,
                            gyro.x.as_rpm() as f32, 
                            gyro.y.as_rpm() as f32, 
                            gyro.z.as_rpm() as f32];
                sample = Sample{sensory: sensory, time: Instant::now(), 
                                read: reading};
                ybsu::SINK.send(to_ytf(sample)).await;
                };
            ticker.next().await;
            };
        }


        use xca9548a::{Xca9548a, SlaveAddr};
        #[embassy_executor::task]
        pub async fn multi_task(i2c: i2c::I2c<'static, ThisI2C>,
                                n: u8,
                                hz: u64, 
                                just_spin: bool,
                                sensory: u8) { 
            
            let tca = Xca9548a::new(i2c, SlaveAddr::default());
            let hub = tca.split();
            let sen_0 = Lsm6::new(hub.i2c0, SlaveAddress::Low, time::Delay);
            let sen_1 = Lsm6::new(hub.i2c1, SlaveAddress::Low, time::Delay);
            let sen_2 = Lsm6::new(hub.i2c2, SlaveAddress::Low, time::Delay);
            let sen_3 = Lsm6::new(hub.i2c3, SlaveAddress::Low, time::Delay);
            let sen_4 = Lsm6::new(hub.i2c4, SlaveAddress::Low, time::Delay);
            let sen_5 = Lsm6::new(hub.i2c5, SlaveAddress::Low, time::Delay);
            let sen_6 = Lsm6::new(hub.i2c6, SlaveAddress::Low, time::Delay);
            let sen_7 = Lsm6::new(hub.i2c7, SlaveAddress::Low, time::Delay);
            let mut sensors = [sen_0, sen_1, sen_2, sen_3, sen_4, sen_5, sen_6, sen_7];
            //let mut sensory = [Some(sen_0), Some(sen_1), Some(sen_2), Some(sen_3), Some(sen_4), Some(sen_5), Some(sen_6), Some(sen_7)];
            let data_rate = DataRate::Freq6660Hz;
            for (s, sens) in sensors.as_mut().into_iter().enumerate() {
                if s >= n as usize {continue}
                else {  sens.set_accel_sample_rate(data_rate).unwrap();
                        sens.set_gyro_sample_rate(data_rate).unwrap();};
            }
            
            let mut ticker 
                    = Ticker::every(Duration::from_hz(hz));
            let mut reading: Reading;
            let mut sample: Sample;
            READY.store(true, RLX);
            loop {
                if SAMPLE.load(RLX){
                    for (s, sensor) in sensors.as_mut().into_iter().enumerate() {
                        if s >= n as usize {continue}
                        let accel = sensor.accel_norm().unwrap();
                        let gyro = sensor.angular_rate().unwrap();
                        reading = [ accel.x, accel.y, accel.z,
                                gyro.x.as_rpm() as f32, 
                                gyro.y.as_rpm() as f32, 
                                gyro.z.as_rpm() as f32];
                        sample = Sample{sensory: sensory + s as u8, time: Instant::now(), read: reading};
                        ybsu::SINK.send(to_ytf(sample)).await;
                        }
                        if !just_spin {ticker.next().await;};
                    };
                }
            }     
    }