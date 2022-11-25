#![feature(inline_const)]
#![feature(variant_count)]
#![feature(async_fn_in_trait)]
#![feature(future_join)]
#![allow(incomplete_features)]
#![no_std]

use embedded_graphics::mono_font::iso_8859_1::{FONT_5X7, FONT_6X10};
use embedded_graphics::mono_font::{MonoFont, MonoTextStyle, MonoTextStyleBuilder};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::pixelcolor::BinaryColor::{Off, On};
use embedded_graphics::primitives::PrimitiveStyle;

pub mod button;
pub mod key;
pub mod notify;
pub mod text;
pub mod widget;

mod backend;

#[cfg(not(any(feature = "simulator", feature = "sharp")))]
compile_error!(
    "You must select either the `simulator` or `sharp` feature in order to compile this crate"
);

// TODO
#[cfg(feature = "sharp")]
compile_error!("sharp isn't implemented yet");

#[cfg(feature = "simulator")]
pub use backend::simulator as display;

pub const SMALL_FONT: &MonoFont = &FONT_5X7;
pub const FONT: &MonoFont = &FONT_6X10;

pub fn thin_line() -> PrimitiveStyle<BinaryColor> {
    PrimitiveStyle::with_stroke(On, 1)
}

pub fn thin_line_off() -> PrimitiveStyle<BinaryColor> {
    PrimitiveStyle::with_stroke(Off, 1)
}

pub fn text<'a>(font: &'a MonoFont<'a>) -> MonoTextStyle<'a, BinaryColor> {
    MonoTextStyle::new(font, On)
}

pub fn text_inverted<'a>(font: &'a MonoFont<'a>) -> MonoTextStyle<'a, BinaryColor> {
    MonoTextStyleBuilder::new()
        .font(font)
        .background_color(On)
        .text_color(Off)
        .build()
}
