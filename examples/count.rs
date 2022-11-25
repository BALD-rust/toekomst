#![feature(type_alias_impl_trait)]
#![feature(async_closure)]
#![feature(future_join)]
#![feature(cell_update)]

use embassy_executor::Spawner;
use embassy_futures::join::join4;
use embassy_futures::select::select;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::SimulatorDisplay;

use auwaa::button::Button;
use auwaa::key::Accel;
use auwaa::notify::Notify;
use auwaa::text::label_with;
use auwaa::widget::Widget;

async fn ui() {
    let a = Accel::new();

    #[derive(Clone, Debug)]
    enum Cmd {
        Plus,
        Min,
    }

    let cmd_notif = Notify::new();
    let text_notif = Notify::new_preoccupied("Count: 0".to_string());

    let pls_pos = Point::new(10, 10);
    let (btn_plus, a) = Button::new(a, pls_pos, "Add 1", &cmd_notif, Cmd::Plus);

    let min_pos = pls_pos + btn_plus.size().x_axis() + Point::new(10, 0);
    let (btn_min, _a) = Button::new(a, min_pos, "Subtract 1", &cmd_notif, Cmd::Min);

    let lbl_pos = btn_plus.position() + btn_plus.size().y_axis();
    let count_label = label_with(&text_notif, lbl_pos + Point::new(0, 10));

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
    env_logger::Builder::from_default_env()
        .format_timestamp_millis()
        .init();

    auwaa::display::init_disp(SimulatorDisplay::new(Size::new(400, 240)));

    select(auwaa::display::run_disp(), ui()).await;

    std::process::exit(0);
}
