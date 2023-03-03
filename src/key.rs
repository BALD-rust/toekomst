use std::sync::atomic::{AtomicBool, Ordering};

use announcement::Announcement;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::signal::Signal;
use strum_macros::IntoStaticStr;

static KEY_CHAN: Announcement<Key> = Announcement::new();
static INHIBITING: AtomicBool = AtomicBool::new(false);
static INHIBITOR: Signal<ThreadModeRawMutex, Key> = Signal::new();

pub async fn wait(k: Key) {
    while KEY_CHAN.recv().await != k {}
}

pub async fn wait_inhibiting() -> Key {
    INHIBITING.store(true, Ordering::Relaxed);
    let key = INHIBITOR.wait().await;
    INHIBITING.store(false, Ordering::Relaxed);

    key
}

pub fn press_key(k: Key) {
    if INHIBITING.load(Ordering::Relaxed) {
        INHIBITOR.signal(k)
    } else {
        KEY_CHAN.announce(k)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Accel(Key);

impl Accel {
    pub fn new() -> Self {
        Accel(Key::a)
    }

    pub fn next(self) -> (Key, Accel) {
        let Accel(key) = self;

        const KEY_LEN: usize = core::mem::variant_count::<Key>();

        // assert we haven't ran out of keys
        debug_assert_ne!(key as u8, KEY_LEN as u8 - 1);

        unsafe { (key, Accel(Key::from_u8(key as u8 + 1))) }
    }
}

impl From<Key> for Accel {
    fn from(value: Key) -> Self {
        Self(value)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, IntoStaticStr)]
#[repr(u8)]
#[allow(non_camel_case_types)]
#[allow(unused)]
pub enum Key {
    a,
    b,
    c,
    d,
    e,
    f,
    g,
}

impl Key {
    /// Safety: `d` must be strictly smaller than [KEY_LEN]
    pub unsafe fn from_u8(d: u8) -> Self {
        core::mem::transmute(d)
    }
}
