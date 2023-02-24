use core::future::Future;
use crossbeam_queue::SegQueue;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use strum_macros::IntoStaticStr;

const KEY_LEN: usize = core::mem::variant_count::<Key>();
static KEY_QUEUE: SegQueue<Key> = SegQueue::new();
// static KEYS: [Signal<CriticalSectionRawMutex, ()>; KEY_LEN] = [const { Signal::new() }; KEY_LEN];

fn get_sig(k: Key) -> &'static Signal<CriticalSectionRawMutex, ()> {
    unsafe { KEYS.get_unchecked(k as usize) }
}

pub fn wait(k: Key) -> impl Future<Output = ()> + 'static {
    let sig = get_sig(k);
    sig.reset();

    sig.wait()
}

pub fn press_key(k: Key) {
    get_sig(k).signal(())
}

#[derive(Copy, Clone, Debug)]
pub struct Accel(Key);

impl Accel {
    pub fn new() -> Self {
        Accel(Key::a)
    }

    pub fn next(self) -> (Key, Accel) {
        let Accel(key) = self;

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

#[derive(Copy, Clone, Debug, IntoStaticStr)]
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
