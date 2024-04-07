pub use super::*;
use core::fmt::Write;

//enum Error{NotImplemented, Send}

pub type Ytf = Sample<[Option<f32>; 8]>; // standard transport format
type YtfLine = Vec<u8, 512>;

trait YtfSend{
    fn msg_csv(&self) -> Result<YtfLine, core::fmt::Error>;
    fn msg_bin(&self) -> Result<YtfLine, core::fmt::Error>;
}

//use core::fmt::Formatter;

impl core::fmt::Display for Ytf {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{}, {}", self.time.as_micros(), self.sensory).unwrap();
            for r in self.read {
                match r {
                    Some(v) => {write!(f, ",{}", v).unwrap();},
                    None => {write!(f, ",").unwrap();}
                }
            }
            writeln!(f, "")
    }
}

impl YtfSend for Ytf {
    fn msg_csv(&self) -> Result<YtfLine, core::fmt::Error>{
        let mut msg: YtfLine = Vec::new();
        match core::write!(&mut msg, "{}", self) {
            Ok(_) => return Ok(msg),
            Err(e) => return Err(e)
       }
        
        /*let mut msg: YtfLine = Vec::new();
        let mut reads:[&str; 8]  = [""; 8];
        match core::write!(&mut msg,
                            "{},{},{},{},{},{},{},{},{},{}\n",
                            self.time.as_micros(), self.sensory, 
                            reads[0], reads[1], reads[2], reads[3],
                            reads[4], reads[5], reads[6], reads[7],) 
            {
                Ok(_) => return Ok(msg),
                Err(_) => return Err(core::fmt::Error),
            }*/
    }

    fn msg_bin(&self) -> Result<YtfLine, core::fmt::Error>{
        todo!()
    }
}

impl YtfSend for Sample<[f32; 3]> {
    fn msg_csv(&self) -> Result<YtfLine, core::fmt::Error>{
        let mut msg: YtfLine = Vec::new();
        match core::write!(&mut msg,
                            "{},{},{},{},{}\n",
                            self.time.as_micros(), self.sensory, 
                            self.read[0], self.read[1], 
                            self.read[2], ) 
                            {
            Ok(_) => return Ok(msg),
            Err(_) => return Err(core::fmt::Error),
        }
        
    }
        //return Ok(line)
    

    fn msg_bin(&self) -> Result<YtfLine, core::fmt::Error>{
        todo!()
    }
}


pub mod bsu {
    pub use super::*;
    //use core::str::from_utf8;

    //pub use super::super::{hal, Sample, Channel, Mutex, Ordering};
    // use super::YtfSend;

    // Channel
    pub static SINK: Channel<RawMutex, Ytf, 2> = Channel::new();
    
    // USB
    use hal::usart::Uart;
    use hal::peripherals;
    //use heapless::Vec;
    
    #[embassy_executor::task]
    pub async fn task(mut usart: Uart<'static, 
                      peripherals::USART2, 
                      peripherals::DMA1_CH6>) {
        loop {
            let sample: Ytf = SINK.receive().await;
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