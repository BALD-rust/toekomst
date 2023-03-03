use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::{Mutex, MutexGuard};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics_simulator::sdl2::Keycode;
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use core::mem::MaybeUninit;
use embassy_futures::yield_now;
use embedded_graphics::prelude::Size;

use crate::key::Key;
use crate::{should_redraw, unrequest_redraw};

pub const ON: BinaryColor = BinaryColor::On;
pub const OFF: BinaryColor = BinaryColor::Off;

static mut DISP: MaybeUninit<Mutex<ThreadModeRawMutex, SimulatorDisplay<BinaryColor>>> =
    MaybeUninit::uninit();

/// Safety: may only be called after display is initialised
pub async fn disp() -> MutexGuard<'static, ThreadModeRawMutex, SimulatorDisplay<BinaryColor>> {
    unsafe { DISP.assume_init_ref() }.lock().await
}

pub fn init_disp(size: Size) {
    unsafe {
        DISP.write(Mutex::new(SimulatorDisplay::new(size)));
    }
}

pub async fn run_disp() {
    let settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::LcdWhite)
        .pixel_spacing(0)
        .scale(2)
        .build();

    let mut window = Window::new("toekomst", &settings);
    window.update(&*disp().await);

    'inf: loop {
        for e in window.events() {
            match e {
                SimulatorEvent::KeyDown { keycode, .. } => {
                    let k = match keycode {
                        Keycode::A => Key::a,
                        Keycode::B => Key::b,
                        Keycode::C => Key::c,
                        Keycode::D => Key::d,
                        Keycode::E => Key::e,
                        Keycode::F => Key::f,
                        Keycode::G => Key::g,
                        _ => continue,
                    };

                    crate::key::press_key(k);
                }
                SimulatorEvent::Quit => break 'inf,
                _ => {}
            }
        }

        yield_now().await;

        if should_redraw() {
            unrequest_redraw();
            window.update(&*disp().await);
        }
    }
}
