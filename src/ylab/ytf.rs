/* YLab transport formats  */

pub mod bsu {
    /* #[derive(Serialize, Deserialize, Debug)]
    pub enum Sensor {None, Adc1_1}*/
    use super::super::Sample;

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
            let msg = sample.to_csv();
            usart.write(&msg.as_bytes()).await.unwrap();
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