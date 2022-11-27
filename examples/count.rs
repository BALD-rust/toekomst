#![feature(type_alias_impl_trait)]
#![feature(async_closure)]

use embassy_executor::Spawner;
use embassy_futures::join::join4;
use embassy_futures::select::select;
use embedded_graphics::prelude::*;
use env_logger::Env;

use toekomst::button::Button;
use toekomst::key::Accel;
use toekomst::label::label_with;
use toekomst::layout::Vertical;
use toekomst::notify::Notify;
use toekomst::widget::Widget;

async fn ui() {
    let ac = Accel::new();
    let mut v = Vertical::new(Point::new(10, 10), 10);

    #[derive(Clone, Debug)]
    enum Cmd {
        Plus,
        Min,
    }

    let cmd_notif = Notify::new();
    let text_notif = Notify::new_preoccupied("Count: 0".to_string());

    let (btn_plus, ac) = Button::new(ac, v.current(), "Add 1", &cmd_notif, Cmd::Plus);
    let (btn_min, _ac) = Button::new(
        ac,
        v.push(btn_plus.size()),
        "Subtract 1",
        &cmd_notif,
        Cmd::Min,
    );
    let count_label = label_with(&text_notif, v.push(btn_min.size()));

    let cmd_fut = async {
        let mut count = 0;

        loop {
            match cmd_notif.wait().await {
                Cmd::Plus => count += 1,
                Cmd::Min => count -= 1,
            };

            text_notif.notify(format!("Count: {}", count));
        }
    };

    let _ = join4(btn_plus.render(), btn_min.render(), count_label, cmd_fut).await;
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
