use core::cell::OnceCell;

use embassy_futures::yield_now;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_nrf::interrupt;
use embassy_nrf::interrupt::InterruptExt;
use embassy_nrf::peripherals::{P0_02, P0_03, P0_13, P0_14, SPI2};
use embassy_nrf::spim::{Config, Spim};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::{Mutex, MutexGuard};
use embassy_sync::signal::Signal;
use embedded_graphics::geometry::Size;
use embedded_graphics::pixelcolor::BinaryColor;
use sharp_memory_display::MemoryDisplay;
use log::info;

pub const ON: BinaryColor = BinaryColor::Off;
pub const OFF: BinaryColor = BinaryColor::On;

static mut DISP: OnceCell<
    Mutex<
        ThreadModeRawMutex,
        MemoryDisplay<Spim<'static, SPI2>, Output<'static, P0_03>, Output<'static, P0_02>>,
    >,
> = OnceCell::new();

static SIG: Signal<ThreadModeRawMutex, ()> = Signal::new();

pub async fn disp() -> MutexGuard<
    'static,
    ThreadModeRawMutex,
    MemoryDisplay<Spim<'static, SPI2>, Output<'static, P0_03>, Output<'static, P0_02>>,
> {
    unsafe { DISP.get().unwrap_unchecked() }.lock().await
}

pub fn init_disp(spi: SPI2, sck: P0_14, mosi: P0_13, cs: P0_03, disp: P0_02, _: Size) {
    let irq = interrupt::take!(SPIM2_SPIS2_SPI2);
    irq.set_priority(interrupt::Priority::P7);
    let spi: Spim<SPI2> = Spim::new_txonly(spi, irq, sck, mosi, Config::default());

    let cs = Output::new(cs, Level::Low, OutputDrive::Standard);
    let disp = Output::new(disp, Level::Low, OutputDrive::Standard); // disp != mosi/di (data in)

    let mut display = MemoryDisplay::new(spi, cs, disp);

    display.clear();
    display.enable();
    display.flush_buffer();

    unsafe {
        DISP.set(Mutex::new(display)).unwrap_unchecked();
    }
}

pub fn request_redraw() {
    SIG.signal(());
}

pub async fn run_disp() {
    loop {
        SIG.wait().await;
        {
            let mut display = disp().await;
            display.flush_buffer();
            info!("refreshed disp");
        }
        yield_now().await
    }
}
