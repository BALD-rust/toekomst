#![feature(type_alias_impl_trait)]
#![feature(async_closure)]

use embassy_executor::Spawner;
use embassy_futures::join::{join, join3, join4, join_array};
use embassy_futures::select::select;
use embedded_graphics::prelude::*;
use env_logger::Env;

use toekomst::{FONT, text};
use toekomst::button::Button;
use toekomst::input::Input;
use toekomst::key::Accel;
use toekomst::label::{label_once, label_with};
use toekomst::layout::{Horizontal, Vertical};
use toekomst::notify::Notify;
use toekomst::widget::Widget;

async fn ui() {
    let ac = Accel::new();
    let mut v = Vertical::new(Point::new(10, 10), 10);

    #[derive(Clone, Debug, PartialEq)]
    enum Cmd {
        One,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Zero,
        Plus,
        Minus,
        Mult,
        Div,
        Dot,
        Back,
        Eq,
    }

    let cmd_notif = Notify::new();
    let text_notif = Notify::new_preoccupied("5".to_string());
    let label = label_with(&text_notif, v.current());

    let mut h = Horizontal::new(v.current(), 10);
    let (btn_eq, ac) = Button::new(ac, h.push(Size::new(75, 0)), "=", &cmd_notif, Cmd::Eq);

    let mut h = Horizontal::new(v.push(FONT.character_size), 10);
    let (btn_1, ac) = Button::new(ac, h.current(), "1", &cmd_notif, Cmd::One);
    let (btn_2, ac) = Button::new(ac, h.push(btn_1.size()), "2", &cmd_notif, Cmd::Two);
    let (btn_3, ac) = Button::new(ac, h.push(btn_1.size()), "3", &cmd_notif, Cmd::Three);
    let (btn_plus, ac) = Button::new(ac, h.push(btn_1.size()), "+", &cmd_notif, Cmd::Plus);
    let (btn_4, ac) = Button::new(ac, v.push(btn_1.size()), "4", &cmd_notif, Cmd::Four);
    let mut h = Horizontal::new(v.current(), 10);
    let (btn_5, ac) = Button::new(ac, h.push(btn_1.size()), "5", &cmd_notif, Cmd::Five);
    let (btn_6, ac) = Button::new(ac, h.push(btn_1.size()), "6", &cmd_notif, Cmd::Six);
    let (btn_minus, ac) = Button::new(ac, h.push(btn_1.size()), "-", &cmd_notif, Cmd::Minus);
    let (btn_7, ac) = Button::new(ac, v.push(btn_1.size()), "7", &cmd_notif, Cmd::Seven);
    let mut h = Horizontal::new(v.current(), 10);
    let (btn_8, ac) = Button::new(ac, h.push(btn_1.size()), "8", &cmd_notif, Cmd::Eight);
    let (btn_9, ac) = Button::new(ac, h.push(btn_1.size()), "9", &cmd_notif, Cmd::Nine);
    let (btn_mult, ac) = Button::new(ac, h.push(btn_1.size()), "*", &cmd_notif, Cmd::Mult);
    let (btn_comma, ac) = Button::new(ac, v.push(btn_1.size()), "<", &cmd_notif, Cmd::Back);
    let mut h = Horizontal::new(v.current(), 10);
    let (btn_0, ac) = Button::new(ac, h.push(btn_1.size()), "0", &cmd_notif, Cmd::Zero);
    let (btn_dot, ac) = Button::new(ac, h.push(btn_1.size()), ".", &cmd_notif, Cmd::Dot);
    let (btn_div, ac) = Button::new(ac, h.push(btn_1.size()), "/", &cmd_notif, Cmd::Div);


    // let (btn_min, _ac) = Button::new(
    //     ac,
    //     v.push(btn_plus.size()),
    //     "Subtract 1",
    //     &cmd_notif,
    //     Cmd::Min,
    // );
    // let count_label = label_with(&text_notif, v.push(btn_min.size()));
    //
    let cmd_fut = async {
        let mut str: String = String::new();

        loop {
            let cmd = cmd_notif.wait().await;
            let input = match cmd {
                Cmd::Plus => "+",
                Cmd::One => "1",
                Cmd::Two => "2",
                Cmd::Three => "3",
                Cmd::Four => "4",
                Cmd::Five => "5",
                Cmd::Six => "6",
                Cmd::Seven => "7",
                Cmd::Eight => "8",
                Cmd::Nine => "9",
                Cmd::Zero => "0",
                Cmd::Minus => "-",
                Cmd::Mult => "*",
                Cmd::Div => "/",
                Cmd::Dot => ".",
                _ => "",
            };

            if cmd == Cmd::Eq {
                str.clear();
                str.push('5');
            }

            if cmd == Cmd::Back {
                str.remove(str.len()-1);
            }

            str.push_str(input.into());

            text_notif.notify(str.clone());
        }
    };

    let _ = join3(join_array([btn_1.render(),
        btn_2.render(),
        btn_3.render(),
        btn_4.render(),
        btn_5.render(),
        btn_6.render(),
        btn_7.render(),
        btn_8.render(),
        btn_9.render(),
        btn_comma.render(),
        btn_0.render(),
        btn_dot.render(),
        btn_plus.render(),
        btn_minus.render(),
        btn_mult.render(),
        btn_div.render(),
        btn_eq.render(),

    ]), label, cmd_fut).await;

    // let _ = join4(btn_plus.render(), btn_min.render(), count_label, cmd_fut).await;
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
