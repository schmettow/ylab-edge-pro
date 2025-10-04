#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Speed};
use embassy_stm32::peripherals::{DMA2_CH2, DMA2_CH3, PA4, PA5, PA6, PA7, SPI1};
use embassy_stm32::spi::{Config as SpiConfig, Spi};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Timer};
use panic_probe as _;
use static_cell::StaticCell;

use ads129x::{Ads129x, ConfigRegisters, Error};

// Für Logging / Defmt
use defmt_rtt as _;
use panic_probe as _;

type SpiBus = Spi<'static, SPI1, DMA2_CH3, DMA2_CH2>;
type SpiBusMutex = Mutex<NoopRawMutex, SpiBus>;

// Static container for the bus (required by SpiDevice::new example pattern)
static SPI_BUS: StaticCell<SpiBusMutex> = StaticCell::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::info!("STM32F446ZE ADS129x example init");

    // init peripherals
    let p = embassy_stm32::init(Default::default());

    // pins (be sure these match your wiring)
    let sck = p.PA5;
    let mosi = p.PA7;
    let miso = p.PA6;
    let cs_pin = p.PA4;

    // SPI config
    let spi_cfg = SpiConfig::default();
    /*spi_cfg.frequency = 1_000_000;
    spi_cfg.phase = embassy_stm32::spi::Phase::CaptureOnFirstTransition;
    spi_cfg.polarity = embassy_stm32::spi::Polarity::IdleLow;*/

    // --- IMPORTANT: pick DMA channels that are valid for SPI1 on F446ZE ---
    // Example channels — VERIFY these against `p` for your specific chip!
    // On many F4 parts SPI1 uses DMA2 streams/channels; check `embassy_stm32::peripherals`.
    let tx_dma = p.DMA2_CH3; // <-- verify on your device
    let rx_dma = p.DMA2_CH2; // <-- verify on your device

    // create the async Spi driver (this returns Spi<'d, Async>)
    let spi = Spi::new(p.SPI1, sck, mosi, miso, tx_dma, rx_dma, spi_cfg);

    // wrap the spi into an embassy_sync::Mutex so it can be shared
    let spi_bus = Mutex::new(spi);

    // make it 'static: initialize the StaticCell
    let spi_bus = SPI_BUS.init(spi_bus);

    // create a CS output pin (adjust constructor to your HAL's Output API)
    // embassy_stm32's Output::new signature may require OutputDrive; check your version.
    let cs = embassy_stm32::gpio::Output::new(cs_pin, Level::High, Speed::High);

    // CORRECT: construct a SpiDevice on top of the shared bus (not Device::new(spi,...))
    let spi_dev = SpiDevice::new(spi_bus, cs);

    // now create the ADS driver with the SpiDevice
    let mut ads = Ads129x::new(spi_dev);

    // apply async config
    let cfg = ConfigRegisters::default();
    if let Err(e) = cfg.apply_async(&mut ads).await {
        //defmt::error!("ADS129x configuration failed: {}", e);
        loop {
            Timer::after(Duration::from_secs(1)).await;
        }
    }

    defmt::info!("Start read loop");
    loop {
        match ads.read_data_2ch_async().await {
            Ok(data) => {
                defmt::info!(
                    "CH1: {} V, CH2: {} V",
                    data.ch1_sample().voltage(),
                    data.ch2_sample().voltage()
                );
            }
            Err(_) => {}
        }
        Timer::after(Duration::from_millis(100)).await;
    }
}
