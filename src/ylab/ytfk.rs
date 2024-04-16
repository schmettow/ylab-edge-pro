pub use super::*;
use core::fmt::Write;


pub mod bsu {
    pub use super::*;

    // Channel
    pub static SINK: Channel<RawMutex, Ytf, 2> = Channel::new();
    
    // USB
    use hal::usart::Uart;
    use hal::peripherals;
    
    #[embassy_executor::task]
    pub async fn task(mut usart: Uart<'static, 
                      peripherals::USART2, 
                      peripherals::DMA1_CH6>) {
        loop {
            let sample: Ytf = SINK.receive().await;
            let mut msg: Vec<u8, 256> = Vec::new();
            match core::write!(&mut msg, "{}", sample) {
                Ok(_) => usart.write(&msg).await.unwrap(),
                Err(_) => {},
            }
            usart.write(&msg).await.unwrap()
        }
    }
}

