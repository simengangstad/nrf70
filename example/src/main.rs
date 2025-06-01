#![no_std]
#![no_main]
#![macro_use]
#![deny(unused_must_use)]
#![allow(async_fn_in_trait)]

use align_data::{include_aligned, Align16};
use defmt::*;
use defmt_rtt as _; // global logger
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pin, Pull};
use embassy_nrf::peripherals::SERIAL0;
use embassy_nrf::spim::Spim;
use embassy_nrf::{bind_interrupts, spim};
use embassy_time::{Delay, Duration, Timer};
use embedded_hal_bus::spi::ExclusiveDevice;
use nrf70::bus::SpiBus;
use static_cell::StaticCell;
use {embassy_nrf as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    SERIAL0 => spim::InterruptHandler<embassy_nrf::peripherals::SERIAL0>;
});

type Nrf70SpiBus = SpiBus<ExclusiveDevice<Spim<'static, SERIAL0>, Output<'static>, Delay>>;

#[embassy_executor::task]
async fn nrf70_task(mut runner: nrf70::Runner<'static, Nrf70SpiBus, Input<'static>, Output<'static>>) -> ! {
    runner.run().await
}

static FW: &[u8] = include_aligned!(Align16, "../../thirdparty/default.bin");

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let config: embassy_nrf::config::Config = Default::default();
    let p = embassy_nrf::init(config);

    let sck = p.P0_17;
    let cs = p.P0_18;

    let data0 = p.P0_13;
    let data1 = p.P0_14;
    let _data2 = p.P0_15;
    let _data3 = p.P0_16;

    //let coex_req = Output::new(p.P0_28, Level::High, OutputDrive::Standard);
    //let coex_status0 = Output::new(p.P0_30, Level::High, OutputDrive::Standard);
    //let coex_status1 = Output::new(p.P0_29, Level::High, OutputDrive::Standard);
    //let coex_grant = Output::new(p.P0_24, Level::High, OutputDrive::Standard);
    let bucken = Output::new(p.P0_12.degrade(), Level::Low, OutputDrive::HighDrive);
    let iovdd_ctl = Output::new(p.P0_31.degrade(), Level::Low, OutputDrive::Standard);
    let host_irq = Input::new(p.P0_23.degrade(), Pull::None);

    let mut config = spim::Config::default();
    config.frequency = spim::Frequency::M1;
    let spim = Spim::new(p.SERIAL0, Irqs, sck, data1, data0, config);
    let cs = Output::new(cs, Level::High, OutputDrive::HighDrive);
    let spi = ExclusiveDevice::new(spim, cs, Delay).unwrap();
    let bus = SpiBus::new(spi);

    /*
    // QSPI is not working well yet.
    let mut config = qspi::Config::default();
    config.read_opcode = qspi::ReadOpcode::READ4IO;
    config.write_opcode = qspi::WriteOpcode::PP4IO;
    config.write_page_size = qspi::WritePageSize::_256BYTES;
    config.frequency = qspi::Frequency::M8; // NOTE: Waking RPU works reliably only with lowest frequency (8MHz)

    let irq = interrupt::take!(QSPI);
    let qspi: qspi::Qspi<_> = qspi::Qspi::new(p.QSPI, irq, sck, csn, dio0, dio1, dio2, dio3, config);
    let bus = QspiBus { qspi };
    */

    static STATE: StaticCell<nrf70::State> = StaticCell::new();
    let state = STATE.init(nrf70::State::new());

    let (_device, mut control, runner) = nrf70::new(state, bus, bucken, iovdd_ctl, host_irq).await;
    unwrap!(spawner.spawn(nrf70_task(runner)));

    match control.init(FW).await {
        Ok(()) => (),
        Err(error) => error!("Failed to initialize {:?}", error),
    };

    // Timer::after_millis(5000).await;

    // match control.get_scan_results().await {
    //     Ok(()) => info!("Requested scan results"),
    //     Err(error) => error!("Failed to request scan results {}", error),
    // }

    // match control.get_stats().await {
    //     Ok(()) => info!("Requested stats"),
    //     Err(error) => error!("Failed to request stats {}", error),
    // }
    //
    // let scan_options = ScanOptions::default();
    //
    // match control.scan(scan_options).await {
    //     Ok(()) => info!("Started scan..."),
    //     Err(error) => error!("Failed to perform scan {}", error),
    // }

    let mut led = Output::new(p.P1_06.degrade(), Level::High, OutputDrive::Standard);
    loop {
        led.set_high();
        Timer::after(Duration::from_millis(100)).await;
        led.set_low();
        Timer::after(Duration::from_millis(100)).await;
    }
}
