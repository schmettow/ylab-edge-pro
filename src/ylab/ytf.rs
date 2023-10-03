use super::{String, Vec};
use core::fmt::{Write, Error};
use super::Sample;

//enum Error{NotImplemented, Send}

type YtfLine = Vec<u8, 128>;

trait YtfSend{
    fn msg_csv(&self) -> Result<Vec<u8, 128>, core::fmt::Error>;
    fn msg_bin(&self) -> Result<Vec<u8, 128>, core::fmt::Error>;
}

/// + methods to create YTF data messages

impl YtfSend for Sample {
    fn msg_csv(&self) -> Result<YtfLine, core::fmt::Error>{
        let mut msg: YtfLine = Vec::new();
        match core::write!(&mut msg,
                            "{},{},{},{},{},{},{},{},{},{}\n",
                            self.time, self.dev, 
                            self.read[0], self.read[1], 
                            self.read[2], self.read[3],
                            self.read[4], self.read[5],
                            self.read[6], self.read[7]) 
                            {
                Ok(_) => return Ok(msg),
                Err(_) => return Err(core::fmt::Error),
                }
        //return Ok(line)
    }

    fn msg_bin(&self) -> Result<YtfLine, core::fmt::Error>{
        let line: Result<YtfLine, _> = postcard::to_vec(&self);
        match line {
            Ok(msg) => return Ok(msg),
            Err(_) => return Err(core::fmt::Error),
        }
    }
}


pub mod bsu {
    //use core::str::from_utf8;

    pub use super::super::{hal, Sample, Channel, Mutex, Ordering};
    use super::YtfSend;

    // Channel
    pub static SINK: Channel<Mutex, Sample, 2> = Channel::new();
    
    // USB
    use hal::usart::Uart;
    use hal::peripherals;
    //use heapless::Vec;
    
    #[embassy_executor::task]
    pub async fn task(mut usart: Uart<'static, 
                      peripherals::USART3, 
                      peripherals::DMA1_CH3>) {
        loop {
            let sample: Sample = SINK.recv().await;
            let msg = sample.msg_csv();
            //let msg = sample.msg_bin();
            match msg {
                Ok(msg) => {usart.write(&msg).await.unwrap()},
                Err(_) => {usart.write("ERR".as_bytes()).await.unwrap()},
            }
        }
    }
}

    /* pub mod y2b {
        /* #[derive(Serialize, Deserialize, Debug)]
        pub enum Sensor {None, Adc1_1}*/
        use super::super::Sample;
        // data handling
        use postcard::to_vec;
        use heapless::Vec;

        // Channel
        use embassy_sync::channel::Channel;
        use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
        pub static SINK: Channel<CriticalSectionRawMutex, Sample, 2> = Channel::new();
        
        // USB
        use embassy_stm32 as hal;
        use hal::usart::Uart;
        use hal::peripherals;
        
        #[embassy_executor::task]
        pub async fn task(mut usart: Uart<'static, peripherals::USART3, peripherals::DMA1_CH3>) {
            loop {
                let sample = SINK.recv().await;
                let ser: Vec<u8, 9> =   to_vec(&sample).unwrap();
                usart.write(&ser).await.unwrap();
                usart.write("\n".as_bytes()).await.unwrap();
                }
            }
        }
    */