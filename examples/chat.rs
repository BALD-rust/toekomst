#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_futures::join::join3;
use embassy_futures::select::select;
use embedded_graphics::geometry::Point;
use embedded_graphics::prelude::Size;
use env_logger::Env;

use toekomst::input::Input;
use toekomst::key::Accel;
use toekomst::label;
use toekomst::label::{label_once, label_with};
use toekomst::layout::Vertical;
use toekomst::notify::Notify;
use toekomst::widget::Widget;

async fn ui() {
    const C_WIDTH: u32 = label::FONT.character_size.width + label::FONT.character_spacing;
    let ac = Accel::new();
    let mut v = Vertical::new(Point::new(10, 10), 10);

    let s = "Chatting as ";
    label_once(s, v.current()).await;
    let name_notif = Notify::new();
    let name = label_with(
        &name_notif,
        // just after the "Chatting as" label
        v.current() + Point::new((s.len() * C_WIDTH as usize) as i32, 0),
    );

    let chat_notif = Notify::new();
    let (input, _ac) = Input::<16>::new(ac, v.push(label::FONT.character_size), &chat_notif);

    let v_top = v;
    let chat_fut = async {
        loop {
            // Chat as Mary

            name_notif.notify("Mary");
            let mary_msg = chat_notif.wait().await;
            let length = mary_msg.len() * C_WIDTH as usize;
            let mut point = v.push(input.size());
            point.x = (400 - 10 - length) as i32;
            label_once(&mary_msg, point).await;
            drop(mary_msg);

            // Chat as Joe

            name_notif.notify("Joe");
            let joe_msg = chat_notif.wait().await;
            label_once(&joe_msg, v.push(label::FONT.character_size)).await;
            drop(joe_msg);
        }
    };

    join3(name, input.render(), chat_fut).await;
}

const fn size_of_ret<F: Fn() -> R, R>(_: &F) -> usize {
    std::mem::size_of::<R>()
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    println!("{}", size_of_ret(&ui));
    let env = Env::default().filter_or("RUST_LOG", "info");

    env_logger::Builder::from_env(env)
        .format_timestamp_millis()
        .init();

    toekomst::display::init_disp(Size::new(400, 240));

    select(toekomst::display::run_disp(), ui()).await;

    std::process::exit(0);
}
