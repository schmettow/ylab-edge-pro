/* YLab transport formats  */

pub mod bsu {
    /* #[derive(Serialize, Deserialize, Debug)]
    pub enum Sensor {None, Adc1_1}*/

    // data handling
    use serde::{Serialize, Deserialize};
    use postcard::to_vec;
    use heapless::Vec;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Obs {
        pub dev: i8,
        pub time: i32,
        pub read: [u32;1],
    }

    // Channel
    use embassy_sync::channel::Channel;
    use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
    pub static SINK: Channel<CriticalSectionRawMutex, Obs, 2> = Channel::new();
    
    // USB
    use embassy_stm32 as hal;
    use hal::usart::Uart;
    use hal::peripherals;
    
    #[embassy_executor::task]
    pub async fn task(mut usart: Uart<'static, peripherals::USART3, peripherals::DMA1_CH3>) {
        loop {
            let obs = SINK.recv().await;
            let line = 
            usart.write(&ser).await.unwrap();
            usart.write("\n".as_bytes()).await.unwrap();
            }
        }
    }

pub mod y2b {
    /* #[derive(Serialize, Deserialize, Debug)]
    pub enum Sensor {None, Adc1_1}*/

    // data handling
    use serde::{Serialize, Deserialize};
    use postcard::to_vec;
    use heapless::Vec;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Obs {
        pub dev: i8,
        pub time: i32,
        pub read: [u32;1],
    }

    // Channel
    use embassy_sync::channel::Channel;
    use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
    pub static SINK: Channel<CriticalSectionRawMutex, Obs, 2> = Channel::new();
    
    // USB
    use embassy_stm32 as hal;
    use hal::usart::Uart;
    use hal::peripherals;
    
    #[embassy_executor::task]
    pub async fn task(mut usart: Uart<'static, peripherals::USART3, peripherals::DMA1_CH3>) {
        loop {
            let obs = SINK.recv().await;
            let ser: Vec<u8, 9> =   to_vec(&obs).unwrap();
            usart.write(&ser).await.unwrap();
            usart.write("\n".as_bytes()).await.unwrap();
            }
        }
    }

