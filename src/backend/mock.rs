use core::borrow::BorrowMut;
use core::cell::LazyCell;
use core::future::poll_fn;
use core::task::Poll;

use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::{Mutex, MutexGuard};
use embedded_graphics::geometry::Size;
use embedded_graphics::mock_display::MockDisplay;
use embedded_graphics::pixelcolor::BinaryColor;

static mut DISP: LazyCell<Mutex<ThreadModeRawMutex, MockDisplay<BinaryColor>>> =
    LazyCell::new(|| {
        let mut display = MockDisplay::new();
        display.set_allow_out_of_bounds_drawing(true);
        Mutex::new(display)
    });

pub async fn disp() -> MutexGuard<'static, ThreadModeRawMutex, MockDisplay<BinaryColor>> {
    unsafe { DISP.borrow_mut() }.lock().await
}

pub fn init_disp(_: Size) {}

pub async fn run_disp() {
    log::warn!("Running mock display, no window will be shown. Enable the `simulator` flag to show the simulator.");

    poll_fn(|_| Poll::<()>::Pending).await;
}
