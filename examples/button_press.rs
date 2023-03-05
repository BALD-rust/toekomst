#![feature(type_alias_impl_trait)]
#![feature(async_closure)]

use std::future::poll_fn;
use std::task::Poll;

use embassy_executor::Spawner;
use embassy_futures::select::select;
use embedded_graphics::prelude::*;
use env_logger::Env;

use toekomst::button::Button;
use toekomst::display::disp;
use toekomst::key::Accel;
use toekomst::label::label_once_on;
use toekomst::notify::Notify;
use toekomst::request_redraw;
use toekomst::widget::{clean_space_on, Widget};

async fn ui() {
    let ac = Accel::new();

    let n = Notify::new();
    let (btn, _ac) = Button::new(ac, Point::new(10, 10), "Btn", &n, ());

    let _ = select(
        btn.render(),
        n.once(|_| async {
            log::info!("Button pressed!");
        }),
    )
    .await;

    {
        let dt = &mut *disp().await;
        clean_space_on(btn.space(), dt);
        label_once_on("Button pressed!", btn.position(), dt);
        request_redraw();
    }

    poll_fn(|_| Poll::Pending).await
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let env = Env::default().filter_or("RUST_LOG", "info");

    env_logger::Builder::from_env(env)
        .format_timestamp_millis()
        .init();

    toekomst::display::init_disp(Size::new(400, 240));

    select(toekomst::display::run_disp(), ui()).await;

    std::process::exit(0);
}
