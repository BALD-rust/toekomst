use std::sync::atomic::{AtomicBool, Ordering};

use announcement::Announcement;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::signal::Signal;
use strum_macros::IntoStaticStr;

static KEY_CHAN: Announcement<u8> = Announcement::new();
static INHIBITING: AtomicBool = AtomicBool::new(false);
static INHIBITOR: Signal<ThreadModeRawMutex, u8> = Signal::new();

pub async fn wait<T: Into<u8>>(key: T) {
    let c = key.into();
    while KEY_CHAN.recv().await != c {}
}

pub async fn wait_inhibiting<T: From<u8>>() -> T {
    INHIBITING.store(true, Ordering::Relaxed);
    let key = INHIBITOR.wait().await;
    INHIBITING.store(false, Ordering::Relaxed);

    T::from(key)
}

pub fn press_key<T: Into<u8>>(key: T) {
    let key = key.into();
    if INHIBITING.load(Ordering::Relaxed) {
        INHIBITOR.signal(key)
    } else {
        KEY_CHAN.announce(key)
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
    h,
    i,
    j,
    k,
    l,
    m,
    n,
    o,
    p,
    q,
    r,
    s,
    t,
    u,
    v,
    w,
    x,
    y,
    z,
    space,
    esc,
    confirm,
    backspace,
    invalid
}

impl From<u8> for Key {
    fn from(value: u8) -> Self {
        const VARIANTS: u8 = core::mem::variant_count::<Key>() as u8;
        if value >= VARIANTS {
            Key::invalid
        } else {
            unsafe { Key::from_u8(value) }
        }
    }
}

impl Into<u8> for Key {
    fn into(self) -> u8 {
        self as u8
    }
}

impl Key {
    /// Safety: `d` must be strictly smaller than [KEY_LEN]
    pub unsafe fn from_u8(d: u8) -> Self {
        core::mem::transmute(d)
    }
}
