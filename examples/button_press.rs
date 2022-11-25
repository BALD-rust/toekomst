#![feature(type_alias_impl_trait)]
#![feature(async_closure)]
#![feature(future_join)]

use std::future::poll_fn;
use std::task::Poll;

use embassy_executor::Spawner;
use embassy_futures::select::select;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::SimulatorDisplay;

use auwaa::button::Button;
use auwaa::display::disp;
use auwaa::key::Accel;
use auwaa::notify::Notify;
use auwaa::text::label_once_on;
use auwaa::widget::{clean_space_on, Widget};

async fn ui() {
    let a = Accel::new();

    let n = Notify::new();
    let (btn, _a) = Button::new(a, Point::new(10, 10), "Btn", &n, ());

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
    }

    poll_fn(|_| Poll::Pending).await
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    env_logger::init();
    auwaa::display::init_disp(SimulatorDisplay::new(Size::new(400, 240)));

    select(auwaa::display::run_disp(), ui()).await;

    std::process::exit(0);
}
