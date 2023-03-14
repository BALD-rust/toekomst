#![feature(inline_const)]
#![feature(variant_count)]
#![feature(async_fn_in_trait)]
#![feature(once_cell)]
#![allow(incomplete_features)]
#![cfg_attr(not(feature = "simulator"), no_std)]

use core::sync::atomic;
use embedded_graphics::mono_font::{MonoFont, MonoTextStyle, MonoTextStyleBuilder};
use embedded_graphics::mono_font::iso_8859_1::{FONT_5X7, FONT_6X12};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::primitives::PrimitiveStyle;

#[cfg(not(any(feature = "simulator", feature = "sharp")))]
pub use backend::mock as display;
#[cfg(feature = "sharp")]
pub use backend::sharp as display;
#[cfg(feature = "simulator")]
pub use backend::simulator as display;

use crate::display::{OFF, ON};

pub mod button;
pub mod common;
pub mod key;
pub mod label;
pub mod layout;
pub mod notify;
pub mod widget;
pub mod input;

mod backend;

extern crate alloc;

pub const SMALL_FONT: &MonoFont = &FONT_5X7;
pub const FONT: &MonoFont = &FONT_6X12;
static REDRAW: atomic::AtomicBool = atomic::AtomicBool::new(true);

pub fn request_redraw() {
    REDRAW.store(true, atomic::Ordering::Relaxed);
}

pub fn unrequest_redraw() {
    REDRAW.store(false, atomic::Ordering::Relaxed);
}

pub fn should_redraw() -> bool {
    REDRAW.load(atomic::Ordering::Relaxed)
}

pub fn thin_line() -> PrimitiveStyle<BinaryColor> {
    PrimitiveStyle::with_stroke(ON, 1)
}

pub fn thin_line_off() -> PrimitiveStyle<BinaryColor> {
    PrimitiveStyle::with_stroke(OFF, 1)
}

pub fn text<'a>(font: &'a MonoFont<'a>) -> MonoTextStyle<'a, BinaryColor> {
    MonoTextStyle::new(font, ON)
}

pub fn text_inverted<'a>(font: &'a MonoFont<'a>) -> MonoTextStyle<'a, BinaryColor> {
    MonoTextStyleBuilder::new()
        .font(font)
        .background_color(ON)
        .text_color(OFF)
        .build()
}

pub const fn character_stride<'a>(font: &'a MonoFont<'a>) -> u32 {
    font.character_size.width + font.character_spacing
}